package de.phyrone.kiwi.gateway

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.KotlinFeature
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.google.common.flogger.FluentLogger
import de.phyrone.kiwi.gateway.socket.GatewaySocket
import de.phyrone.kiwi.gateway.socket.SockJsGatewaySocket
import de.phyrone.kiwi.gateway.socket.WsGatewaySocket
import io.r2dbc.spi.ConnectionFactories
import io.vertx.core.Vertx
import io.vertx.core.VertxOptions
import io.vertx.core.http.HttpServer
import io.vertx.core.http.HttpServerOptions
import io.vertx.ext.web.Router
import io.vertx.ext.web.handler.sockjs.SockJSHandler
import io.vertx.ext.web.handler.sockjs.SockJSHandlerOptions
import io.vertx.kotlin.coroutines.coAwait
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import org.koin.core.Koin
import org.koin.core.KoinApplication
import org.koin.core.context.startKoin
import org.koin.dsl.bind
import org.koin.dsl.module
import picocli.CommandLine
import kotlin.system.exitProcess

object Main {

    private val logger = FluentLogger.forEnclosingClass()

    @JvmStatic
    fun main(args: Array<String>) {
        val commandLine = CommandLine(StartupParams::class.java)
        exitProcess(commandLine.execute(*args))
    }


    private fun components() = module(true) {
        single { Vertx.vertx(getOrNull() ?: VertxOptions()) }
        single {
            ObjectMapper().registerKotlinModule {
                this.enable(KotlinFeature.NullToEmptyMap)
                this.enable(KotlinFeature.NullToEmptyCollection)
            }.findAndRegisterModules()
        } bind ObjectMapper::class
        single {
            SockJSHandler.create(
                get(),
                getOrNull<SockJSHandlerOptions>() ?: SockJSHandlerOptions()
            )
        } bind SockJSHandler::class
    }

    fun runApplication(params: StartupParams) {
        val koinApplication = startKoin {
            modules(module(true) {
                single { params } bind StartupParams::class
                single { this@startKoin } bind KoinApplication::class
                single { get<KoinApplication>().koin } bind Koin::class
            }, components())
            createEagerInstances()
        }
        val koin = koinApplication.koin

        CoroutineScope(MainLoop).launch {
            runServer(koin, params.binds) { socket ->
                GatewaySession(koin, socket).run()
            }
        }

        MainLoop.run_loop()

    }

    private suspend fun runServer(
        koin: Koin,
        binds: List<String>,
        socketHandler: suspend (GatewaySocket) -> Unit
    ): List<HttpServer> {
        val addresses = binds.mapNotNull { addressString ->
            val parsedAddress = parseSocketAddress(addressString)
            if (parsedAddress == null) {
                logger.atSevere().log("Invalid address: $addressString")
            }
            parsedAddress
        }

        if (addresses.isEmpty()) {
            logger.atSevere().log("No addresses to bind provided")
            exitProcess(1)
        }
        val objectMapper = koin.get<ObjectMapper>()
        val vertx = koin.get<Vertx>()
        val sockJsHandler = koin.get<SockJSHandler>()
        val router = Router.router(vertx)
        router.get("/").handler { ctx ->
            handleWebsocket(ctx, { socket ->
                CoroutineScope(Dispatchers.Default).launch {
                    WsGatewaySocket(objectMapper, socket, null).use { socket ->
                        socketHandler(socket)
                    }
                }
            }, {
                ctx.response().setStatusCode(200)
                //TODO gateway hello page
                ctx.response().end("Welcome to the Kiwi Gateway!")
            })
        }
        router.route().subRouter(sockJsHandler.socketHandler { websocket ->
            CoroutineScope(Dispatchers.Default).launch {
                SockJsGatewaySocket(objectMapper, websocket).use { socket ->
                    socketHandler(socket)
                }
            }
        })

        return addresses.map { address ->
            val server = vertx.createHttpServer(
                HttpServerOptions()
                    .setHandle100ContinueAutomatically(true)
                    .setTcpKeepAlive(true)
                    .setTcpFastOpen(true)
            )
            server.requestHandler(router)
            server.listen(address).coAwait()
            logger.atInfo().log("listening to http://${address.host()}:${server.actualPort()}")
            server
        }
    }
}