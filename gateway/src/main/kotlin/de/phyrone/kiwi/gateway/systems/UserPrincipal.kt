package de.phyrone.kiwi.gateway.systems

import de.phyrone.kiwi.gateway.documents.SessionData
import io.ktor.server.auth.Principal

data class UserPrincipal(
    val id: Long,
    val sessionData: SessionData
) : Principal