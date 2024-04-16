package de.phyrone.kiwi.gateway.routes

import com.auth0.jwt.algorithms.Algorithm
import de.mkammerer.argon2.Argon2
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
    public val snowflakeServiceCoroutineStub: SnowflakeServiceGrpcKt.SnowflakeServiceCoroutineStub,
    private val argon2: Argon2
) : WebApplication {


    override fun Application.apply() {
        routing {
            route(API_V1_BASE_PATH) {
                route("auth") {
                    authenticate(optional = true) {
                        get("/check") {
                            val user = call.principal<UserPrincipal>()
                            if (user == null) {
                                call.respond(
                                    HttpStatusCode.OK, AuthCheckResponse.Unauthenticated as AuthCheckResponse
                                )
                            } else {
                                call.respond(
                                    HttpStatusCode.OK, AuthCheckResponse.Authenticated(user.id) as AuthCheckResponse
                                )
                            }
                        }
                    }

                    post("login") {
                        val (username, password) = call.receive<LoginRequest.PasswordLoginRequest>()
                        postgres.connection { connection ->
                            connection.transactionIsolationLevel = IsolationLevel.READ_COMMITTED
                            val databaseSelected = connection.createStatement(
                                """
                                    SELECT id,password,session_secret 
                                    FROM account WHERE email = $1 LIMIT 1
                                    """.trimIndent()
                            ).bind(0, username).fetchSize(1).execute().awaitSingle().map { row ->
                                val accountID = row.get("id") as Long
                                val password = row.get("password", String::class.java) ?: return@map null
                                val sessionSecret = row.get("session_secret", String::class.java)
                                Triple(accountID, password, sessionSecret)
                            }.awaitFirstOrNull()


                            if (databaseSelected == null) {
                                call.respondError(LoginResponse.InvalidCredentials)
                            } else {
                                val (accountID, hashedPassword, sessionSecret) = databaseSelected
                                val passwordMatches =
                                    argon2.verify(hashedPassword, password.toCharArray(), Charsets.UTF_8)
                                if (passwordMatches) {
                                    val token = if (sessionSecret == null) {
                                        sessionManager.createSession(SessionData(accountID))
                                    } else {
                                        sessionManager.createSession(
                                            SessionData(accountID),
                                            Algorithm.HMAC512(sessionSecret)
                                        )
                                    }
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

                            val (hashedPassword, passwordHashTiming) = measureTimedValue {
                                argon2.hash(
                                    8,
                                    128 * 1024 * 1024,
                                    4,
                                    password.toCharArray(),
                                    Charsets.UTF_8
                                )
                            }
                            call.addTiming("hash", passwordHashTiming)
                            val sessionSecret = sessionManager.createSessionSecret()
                            val (accountID, insertTime) = measureTimedValue {
                                val id = snowflakeServiceCoroutineStub.getSnowflakes(
                                    SnowflakeRequest.newBuilder().setCount(1).build()
                                ).snowflakesList.first()
                                connection.createStatement("INSERT INTO account(email,password,id,session_secret) VALUES ($1,$2,$3,$4) ON CONFLICT ON CONSTRAINT uniq_account_email DO NOTHING RETURNING id")
                                    .bind(0, lowercasedUsername)
                                    .bind(1, hashedPassword)
                                    .bind(2, id)
                                    .bind(3, sessionSecret)
                                    .fetchSize(1).execute().awaitFirst()
                                    .map { row -> row.get("id") as Long }.awaitFirstOrNull()
                            }

                            call.addTiming("insert", insertTime)
                            //TODO respond with documents
                            if (accountID == null) {
                                call.respondError(RegisterResponse.UserAlreadyExists(lowercasedUsername))
                            } else {
                                val token = sessionManager.createSession(
                                    SessionData(accountID),
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