package de.phyrone.kiwi.gateway

import io.ktor.server.application.Application
import io.ktor.server.plugins.ratelimit.RateLimitConfig
import io.ktor.server.plugins.statuspages.StatusPagesConfig

interface WebApplication {

    fun RateLimitConfig.apply() {}
    fun StatusPagesConfig.apply() {}

    fun Application.apply()


}