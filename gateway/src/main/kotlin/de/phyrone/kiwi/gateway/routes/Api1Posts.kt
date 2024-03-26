package de.phyrone.kiwi.gateway.routes

import de.phyrone.kiwi.gateway.WebApplication
import de.phyrone.kiwi.gateway.routes.API_V1_BASE_PATH
import io.ktor.server.application.Application
import io.ktor.server.routing.route
import io.ktor.server.routing.routing
import org.koin.core.annotation.Single

@Single
class Api1Posts : WebApplication {
    override fun Application.apply() {
        routing {
            route(API_V1_BASE_PATH) {

            }
        }
    }
}