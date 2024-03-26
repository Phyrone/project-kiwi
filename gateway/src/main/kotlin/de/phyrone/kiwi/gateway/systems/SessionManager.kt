package de.phyrone.kiwi.gateway.systems

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import com.github.benmanes.caffeine.cache.Caffeine
import com.github.benmanes.caffeine.cache.stats.ConcurrentStatsCounter
import com.google.common.io.BaseEncoding
import com.sksamuel.aedile.core.Builder
import com.sksamuel.aedile.core.cacheBuilder
import de.phyrone.kiwi.common.lazyArg
import de.phyrone.kiwi.common.logger
import de.phyrone.kiwi.common.systems.StartupRunner
import de.phyrone.kiwi.database.connection
import de.phyrone.kiwi.gateway.addTiming
import de.phyrone.kiwi.gateway.documents.SessionData
import io.ktor.server.application.ApplicationCall
import io.r2dbc.postgresql.PostgresqlConnectionFactory
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.toSet
import kotlinx.coroutines.launch
import kotlinx.coroutines.reactive.asFlow
import kotlinx.coroutines.reactive.awaitFirst
import kotlinx.coroutines.reactive.awaitFirstOrNull
import kotlinx.coroutines.reactor.awaitSingle
import kotlinx.coroutines.reactor.awaitSingleOrNull
import org.koin.core.annotation.Single
import java.security.SecureRandom
import java.time.Instant
import java.util.*
import kotlin.jvm.optionals.getOrNull
import kotlin.time.Duration.Companion.minutes
import kotlin.time.Duration.Companion.seconds
import kotlin.time.measureTimedValue

@Single(binds = [StartupRunner::class, SessionManager::class])
class SessionManager(
    private val postgres: PostgresqlConnectionFactory,
    private val jsonMapper: ObjectMapper
) : StartupRunner {

    private val secureRandom = SecureRandom.getInstanceStrong()

    //TODO event based cache invalidation (e.g. send event from postgres to invalidate cache)
    private val sessionSecretCache = cacheBuilder<Long, Optional<Algorithm>> {
        expireAfterWrite = 30.minutes
        refreshAfterWrite = 30.seconds
        maximumSize = 10_000
        statsCounter = ConcurrentStatsCounter()
    }.buildAll(::loadSecretsFromDatabase)

    private val sessionTokenCache = cacheBuilder<String, Optional<UserPrincipal>> {
        refreshAfterWrite = 10.seconds
        expireAfterAccess = 30.seconds
        statsCounter = ConcurrentStatsCounter()
    }.build(::loadToken)

    override suspend fun onStartup() {
        logger.atInfo().log("SessionManager started...")


    }


    private suspend fun loadToken(token: String): Optional<UserPrincipal> {
        val decoded = JWT.decode(token)
        val payload = runCatching {
            val payloadJson = String(BaseEncoding.base64().decode(decoded.payload), Charsets.UTF_8)
            jsonMapper.readValue<SessionData>(payloadJson)
        }.getOrNull() ?: return Optional.empty()

        val secret = sessionSecretCache.get(payload.account)
            .getOrNull() ?: return Optional.empty()
        val verified = runCatching {
            JWT.require(secret)
                .build()
                .verify(decoded)
        }.isSuccess
        if (!verified) return Optional.empty()
        return Optional.of(UserPrincipal(payload.account, payload))
    }

    private suspend fun loadSecretsFromDatabase(users: Set<Long>): Map<Long, Optional<Algorithm>> {
        postgres.connection { connection ->
            logger.atFine().log(
                "Loading session secrets for validation %s",
                lazyArg { users.joinToString(limit = 10, prefix = "[", postfix = "]") }
            )
            return connection.createStatement("SELECT id, session_secret FROM account WHERE id = ANY($1) LIMIT $2")
                .bind("$1", users.toTypedArray())
                .bind("$2", users.size)
                .fetchSize(users.size)
                .execute()
                .awaitFirst()
                .map { row ->
                    val id = row.get("id") as Long
                    val secretData = row.get("session_secret", ByteArray::class.java)
                    id to Optional.ofNullable(Algorithm.HMAC512(secretData))
                }.asFlow().toSet().toMap()
        }
    }

    private suspend fun loadOrCreateSecretFromDatabase(account: Long): Algorithm {
        postgres.connection { connection ->
            connection.beginTransaction().awaitSingleOrNull()
            val secretData =
                connection.createStatement("SELECT  session_secret FROM account WHERE id = $1 LIMIT 1 FOR UPDATE ")
                    .bind(0, account)
                    .fetchSize(1)
                    .execute()
                    .awaitFirst()
                    .map { row -> row.get("session_secret", ByteArray::class.java) }
                    .awaitFirstOrNull()

            val secret = if (secretData != null) {
                Algorithm.HMAC512(secretData)
            } else {
                val newSessionToken = createSessionSecret()
                val updated = connection.createStatement("UPDATE account SET session_secret = $1 WHERE id = $2")
                    .bind(0, newSessionToken)
                    .bind(1, account)
                    .execute()
                    .awaitFirst()
                    .rowsUpdated.awaitSingle()
                assert(updated == 1L) { "Failed to update session secret" }
                Algorithm.HMAC512(newSessionToken)
            }
            sessionSecretCache.put(account, Optional.of(secret))
            connection.commitTransaction().awaitSingleOrNull()
            return secret
        }
    }

    private suspend fun getOrCreateSecret(account: Long): Algorithm {
        return sessionSecretCache.getIfPresent(account)
            ?.getOrNull() ?: loadOrCreateSecretFromDatabase(account)
    }

    suspend fun ApplicationCall.verifySession(token: String): UserPrincipal? {
        val (value, time) = measureTimedValue { sessionTokenCache.get(token).getOrNull() }
        addTiming("ses", time)
        return value
    }

    suspend fun createSession(data: SessionData): String {
        val secret = getOrCreateSecret(data.account)
        return createSession(data, secret)
    }

    fun createSession(data: SessionData, secret: Algorithm): String {
        val payloadString = jsonMapper.writeValueAsString(data)
        return JWT.create()
            .withPayload(payloadString)
            .withIssuedAt(Instant.now())
            .sign(secret).also {
                sessionTokenCache.put(it, Optional.of(UserPrincipal(data.account, data)))
            }
    }

    fun createSessionSecret(): ByteArray {
        val newSessionToken = ByteArray(512)
        secureRandom.nextBytes(newSessionToken)
        return newSessionToken
    }

    companion object {
        private val logger = logger()
    }

}