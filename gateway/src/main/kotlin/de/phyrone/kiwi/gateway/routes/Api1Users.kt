package de.phyrone.kiwi.gateway.routes

import de.phyrone.kiwi.gateway.WebApplication
import io.ktor.server.application.Application
import io.ktor.server.locations.KtorExperimentalLocationsAPI
import io.ktor.server.locations.Location
import io.ktor.server.locations.get
import io.ktor.server.routing.route
import io.ktor.server.routing.routing
import org.koin.core.annotation.Single

@Single
class Api1Users : WebApplication {

    @KtorExperimentalLocationsAPI
    @Location("/users/byID/{id}")
    data class UsersRoute(val id: Long)

    @OptIn(KtorExperimentalLocationsAPI::class)
    override fun Application.apply() {
        routing {
            route(API_V1_BASE_PATH) {
                get<UsersRoute>() { route ->

                }
            }
        }
    }
}