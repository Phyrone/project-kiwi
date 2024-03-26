package de.phyrone.kiwi.gateway.systems

import de.phyrone.kiwi.gateway.WebApplication
import io.ktor.server.application.Application
import io.ktor.server.routing.route
import io.ktor.server.routing.routing
import io.ktor.server.websocket.webSocket
import org.koin.core.Koin
import org.koin.core.annotation.Single
import org.koin.core.component.KoinComponent

@Single
class WebSocketSubscriptionService(
    private val koin: Koin,
) : KoinComponent, WebApplication {
    override fun getKoin(): Koin = koin
    override fun Application.apply() {
        routing {
            route("/v1/events") {
                webSocket {

                }
            }
        }
    }


}