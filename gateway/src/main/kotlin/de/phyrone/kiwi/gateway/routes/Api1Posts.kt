@file:OptIn(KtorExperimentalLocationsAPI::class)

package de.phyrone.kiwi.gateway.routes

import de.phyrone.kiwi.gateway.WebApplication
import io.ktor.http.HttpStatusCode
import io.ktor.server.application.Application
import io.ktor.server.application.call
import io.ktor.server.auth.authenticate
import io.ktor.server.locations.KtorExperimentalLocationsAPI
import io.ktor.server.locations.Location
import io.ktor.server.locations.delete
import io.ktor.server.locations.get
import io.ktor.server.response.respond
import io.ktor.server.routing.post
import io.ktor.server.routing.route
import io.ktor.server.routing.routing
import org.koin.core.annotation.Single

@Single
class Api1Posts : WebApplication {


    @Location("/posts/{id}")
    data class PostRoute(
        val id: Long,
        val user: Long? = null,
    )

    @Location("/posts")
    data class AllPosts(
        val page: Int? = null,
        val byUser: Long? = null
    )

    override fun Application.apply() {
        routing {
            route(API_V1_BASE_PATH) {
                authenticate(optional = true) {
                    post {
                        call.respond(HttpStatusCode.NotImplemented)
                    }
                    delete<PostRoute> { route ->
                        call.respond(HttpStatusCode.NotImplemented, route)
                    }
                    get<PostRoute> { route ->
                        call.respond(HttpStatusCode.NotImplemented, route)
                    }
                }
            }
        }
    }
}