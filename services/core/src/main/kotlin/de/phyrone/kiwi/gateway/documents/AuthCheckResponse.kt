package de.phyrone.kiwi.gateway.documents

import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName


@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "outcome")
sealed interface AuthCheckResponse : JsonDocument {
    val authenticated: Boolean
    val success: Boolean

    @JsonTypeName("authenticated")
    data class Authenticated(
        val account: Long
    ) : AuthCheckResponse {
        override val success: Boolean
            get() = true
        override val authenticated: Boolean
            get() = true
    }

    @JsonTypeName("unauthenticated")
    data object Unauthenticated : AuthCheckResponse {
        override val success: Boolean
            get() = true
        override val authenticated: Boolean
            get() = false
    }


}