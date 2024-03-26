package de.phyrone.kiwi.gateway

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.cbor.databind.CBORMapper
import com.fasterxml.jackson.dataformat.xml.XmlMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLMapper
import de.phyrone.kiwi.gateway.documents.HealthStatusResponse
import de.phyrone.kiwi.gateway.systems.SessionManager
import io.ktor.http.ContentType
import io.ktor.http.HttpHeaders
import io.ktor.http.HttpStatusCode
import io.ktor.network.tls.certificates.generateCertificate
import io.ktor.server.application.Application
import io.ktor.server.application.call
import io.ktor.server.application.install
import io.ktor.server.auth.Authentication
import io.ktor.server.auth.bearer
import io.ktor.server.engine.ApplicationEngine
import io.ktor.server.engine.ApplicationEngineEnvironment
import io.ktor.server.engine.applicationEngineEnvironment
import io.ktor.server.engine.connector
import io.ktor.server.engine.embeddedServer
import io.ktor.server.engine.sslConnector
import io.ktor.server.locations.KtorExperimentalLocationsAPI
import io.ktor.server.locations.Locations
import io.ktor.server.netty.Netty
import io.ktor.server.plugins.autohead.AutoHeadResponse
import io.ktor.server.plugins.callid.CallId
import io.ktor.server.plugins.callid.generate
import io.ktor.server.plugins.compression.Compression
import io.ktor.server.plugins.compression.deflate
import io.ktor.server.plugins.compression.gzip
import io.ktor.server.plugins.compression.minimumSize
import io.ktor.server.plugins.contentnegotiation.ContentNegotiation
import io.ktor.server.plugins.cors.routing.CORS
import io.ktor.server.plugins.defaultheaders.DefaultHeaders
import io.ktor.server.plugins.statuspages.StatusPages
import io.ktor.server.plugins.statuspages.StatusPagesConfig
import io.ktor.server.plugins.swagger.swaggerUI
import io.ktor.server.request.uri
import io.ktor.server.response.respond
import io.ktor.server.response.respondOutputStream
import io.ktor.server.response.respondRedirect
import io.ktor.server.routing.get
import io.ktor.server.routing.routing
import io.ktor.server.websocket.WebSockets
import io.ktor.websocket.WebSocketDeflateExtension
import org.koin.core.annotation.Single
import org.msgpack.jackson.dataformat.MessagePackMapper
import java.util.zip.*

@OptIn(KtorExperimentalLocationsAPI::class)
@Single
fun setupWebApp(
    hooks: List<WebApplication>,
    objectMapper: ObjectMapper,
    xmlFactory: XmlMapper,
    cborFactory: CBORMapper,
    msgpackMapper: MessagePackMapper,
    yamlMapper: YAMLMapper,
    sessionManager: SessionManager
) = applicationEngineEnvironment {
    connector {
        port = 7080
    }

    module {
        install(AutoHeadResponse)
        install(DefaultHeaders) {
            this.header("Server", "Kiwi-Gateway")
            this.header("X-Powered-By", "Ktor")
        }
        install(Locations)
        install(StatusPages) statusPages@{
            hooks.forEach { app -> with(app) { this@statusPages.apply() } }
        }
        install(ContentNegotiation) {
            this.checkAcceptHeaderCompliance = true
            jackson(ContentType.Application.Json, true, objectMapper)
            jackson(ContentType.Application.Xml, true, xmlFactory)
            jackson(ContentType.Application.Cbor, true, cborFactory)
            jackson(ContentType.parse("text/yaml"), true, yamlMapper)
            jackson(ContentType.parse("text/x-yaml"), true, yamlMapper)
            jackson(ContentType.parse("application/x-msgpack"), true, msgpackMapper)
            jackson(ContentType.parse("application/msgpack"), true, msgpackMapper)

        }
        //install(RateLimit) { global { rateLimiter(500, 10.seconds) };hooks.forEach { app -> with(app) { apply() } } }
        install(CallId) {
            generate(12)
            this.retrieveFromHeader("X-Request-Id")
            this.replyToHeader("X-Request-Id")
        }
        install(WebSockets) {
            extensions {

                install(WebSocketDeflateExtension) {
                    compressionLevel = Deflater.BEST_COMPRESSION
                    compressIfBiggerThan(4 * 1024)
                }
            }
        }
        install(CORS) {
            anyHost()
            allowHeader(HttpHeaders.ContentType)
        }
        install(Compression) {
            gzip {
                priority = 1.0
                minimumSize(1024)
            }
            deflate {
                priority = 10.0
                minimumSize(1024)
            }
        }
        install(Authentication) {
            bearer {
                this.realm = "Kiwi"
                this.authenticate { token ->
                    with(sessionManager) { verifySession(token.token) }
                }
            }
        }
    }

    hooks.forEach { app -> with(app) { module { apply() } } }
}

@Single(binds = [ApplicationEngine::class])
fun webServer(
    env: ApplicationEngineEnvironment
) = embeddedServer(Netty, env) {
    this.tcpKeepAlive = true
}

@Single
class DefaultStatusPages(
    private val objectMapper: ObjectMapper
) : WebApplication {
    override fun StatusPagesConfig.apply() {
        status(HttpStatusCode.NotFound) { call, httpStatusCode ->
            val response = RFC9457ResourceNotFound(call.request.uri)
            call.respondOutputStream(ContentType.Application.ProblemJson, HttpStatusCode.NotFound) {
                objectMapper.writeValue(this, response)
            }
        }
        status(HttpStatusCode.InternalServerError) { call, httpStatusCode ->
            val response = ExtendedRFC9457Error(
                HttpStatusCode.InternalServerError,
                "https://httpstatuses.com/500",
                "Internal Server Error",
                "An internal server error occurred if you are the administrator please check the logs."
            )
            call.respondOutputStream(ContentType.Application.ProblemJson, HttpStatusCode.NotFound) {
                objectMapper.writeValue(this, response)
            }
        }
    }

    override fun Application.apply() {

        routing {
            //openAPI("/ui","openapi.yaml")
            swaggerUI("/ui", "openapi.yaml")
            get("/") { call.respondRedirect("/ui", true) }
            get("/health") { call.respond(HealthStatusResponse(true)) }
        }
    }
}