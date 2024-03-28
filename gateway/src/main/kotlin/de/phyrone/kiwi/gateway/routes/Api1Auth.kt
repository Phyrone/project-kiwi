package de.phyrone.kiwi.gateway.routes

import com.auth0.jwt.algorithms.Algorithm
import de.phyrone.kiwi.common.crypto.Argon2Raw
import de.phyrone.kiwi.database.connection
import de.phyrone.kiwi.gateway.WebApplication
import de.phyrone.kiwi.gateway.addTiming
import de.phyrone.kiwi.gateway.documents.AuthCheckResponse
import de.phyrone.kiwi.gateway.documents.LoginRequest
import de.phyrone.kiwi.gateway.documents.LoginResponse
import de.phyrone.kiwi.gateway.documents.RegisterRequest
import de.phyrone.kiwi.gateway.documents.RegisterResponse
import de.phyrone.kiwi.gateway.documents.SessionData
import de.phyrone.kiwi.gateway.respondError
import de.phyrone.kiwi.gateway.systems.SessionManager
import de.phyrone.kiwi.gateway.systems.UserPrincipal
import de.phyrone.kiwi.snowflake.SnowflakeRequest
import de.phyrone.kiwi.snowflake.SnowflakeServiceGrpcKt
import io.ktor.http.HttpStatusCode
import io.ktor.server.application.Application
import io.ktor.server.application.call
import io.ktor.server.auth.authenticate
import io.ktor.server.auth.principal
import io.ktor.server.request.receive
import io.ktor.server.response.respond
import io.ktor.server.routing.get
import io.ktor.server.routing.post
import io.ktor.server.routing.route
import io.ktor.server.routing.routing
import io.r2dbc.postgresql.PostgresqlConnectionFactory
import io.r2dbc.spi.IsolationLevel
import kotlinx.coroutines.reactive.awaitFirst
import kotlinx.coroutines.reactive.awaitFirstOrNull
import kotlinx.coroutines.reactive.awaitSingle
import kotlinx.coroutines.reactor.awaitSingleOrNull
import org.koin.core.annotation.Single
import java.time.Instant
import kotlin.time.measureTimedValue


@Single
class Api1Auth(
    private val postgres: PostgresqlConnectionFactory,
    private val sessionManager: SessionManager,
    public val snowflakeServiceCoroutineStub: SnowflakeServiceGrpcKt.SnowflakeServiceCoroutineStub
) : WebApplication {


    override fun Application.apply() {
        routing {
            route(API_V1_BASE_PATH) {
                route("auth") {
                    authenticate(optional = true) {
                        get("/check") {
                            val user = call.principal<UserPrincipal>()
                            if (user == null) {
                                call.respond(HttpStatusCode.OK, AuthCheckResponse.Unauthenticated as AuthCheckResponse)
                            } else {
                                call.respond(
                                    HttpStatusCode.OK,
                                    AuthCheckResponse.Authenticated(user.id) as AuthCheckResponse
                                )
                            }
                        }
                    }

                    post("login") {
                        val (username, password) = call.receive<LoginRequest.PasswordLoginRequest>()

                        postgres.connection { connection ->
                            connection.transactionIsolationLevel = IsolationLevel.READ_COMMITTED
                            val databaseSelected =
                                connection.createStatement(
                                    """
                                    SELECT id,(password).hash AS hash, (password).salt AS salt,(password).iterations AS iterations,(password).memory AS memory,(password).parallelism AS parallelism,session_secret 
                                    FROM account WHERE email = $1 LIMIT 1
                                    """.trimIndent()
                                )
                                    .bind(0, username).fetchSize(1).execute().awaitSingle().map { row ->
                                        val accountID = row.get("id") as Long
                                        val hash = row.get("hash", ByteArray::class.java) ?: return@map null
                                        val salt = row.get("salt", ByteArray::class.java) ?: return@map null
                                        val iterations = row.get("iterations") as? Int ?: return@map null
                                        val memory = row.get("memory") as? Int ?: return@map null
                                        val parallelism = row.get("parallelism") as? Int ?: return@map null
                                        return@map accountID to Argon2Raw.HashedPassword(
                                            hash,
                                            salt,
                                            iterations,
                                            memory,
                                            parallelism
                                        )
                                    }.awaitFirstOrNull()

                            if (databaseSelected == null) {
                                call.respondError(LoginResponse.InvalidCredentials)
                            } else {
                                val (accountID, hashedPassword) = databaseSelected
                                val passwordMatches = Argon2Raw.verify(password, hashedPassword)
                                if (passwordMatches) {
                                    val token = sessionManager.createSession(SessionData(accountID))
                                    call.respond(HttpStatusCode.Accepted, LoginResponse.Success(accountID, token))
                                } else {
                                    call.respondError(LoginResponse.InvalidCredentials)
                                }
                            }
                        }
                    }
                    post("register") {
                        val (username, password) = call.receive<RegisterRequest.PasswordRegisterRequest>()
                        val lowercasedUsername = username.lowercase()
                        postgres.connection { connection ->
                            connection.setAutoCommit(true).awaitSingleOrNull()

                            val (hashed, hashingTime) = measureTimedValue { Argon2Raw.create(password) }
                            call.addTiming("hash", hashingTime)
                            val sessionSecret = sessionManager.createSessionSecret()
                            val (accountID, insertTime) = measureTimedValue {
                                val id = snowflakeServiceCoroutineStub.getSnowflakes(
                                    SnowflakeRequest.newBuilder().setCount(1).build()
                                ).snowflakesList.first()
                                connection.createStatement("INSERT INTO account(email,password,id,session_secret) VALUES ($1,ROW($2,$3,$4,$5,$6),$7,$8) ON CONFLICT ON CONSTRAINT uniq_account_email DO NOTHING RETURNING id")
                                    .bind(0, lowercasedUsername)
                                    .bind(1, hashed.hash)
                                    .bind(2, hashed.salt)
                                    .bind(3, hashed.iterations)
                                    .bind(4, hashed.memory)
                                    .bind(5, hashed.parallelism)
                                    .bind(6, id).bind(7, sessionSecret).fetchSize(1).execute().awaitFirst()
                                    .map { row -> row.get("id") as Long }.awaitFirstOrNull()
                            }

                            call.addTiming("insert", insertTime)
                            //TODO respond with documents
                            if (accountID == null) {
                                call.respondError(RegisterResponse.UserAlreadyExists(lowercasedUsername))
                            } else {
                                val token = sessionManager.createSession(
                                    SessionData(accountID, Instant.now().plusSeconds(600)),
                                    Algorithm.HMAC512(sessionSecret)
                                )
                                call.respond(
                                    HttpStatusCode.Accepted,
                                    RegisterResponse.Success(accountID, lowercasedUsername, token)
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}