package de.phyrone.kiwi.gateway.documents

import com.fasterxml.jackson.annotation.JsonAlias
import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "with")
@JsonSubTypes(
    JsonSubTypes.Type(value = LoginRequest.PasswordLoginRequest::class, name = "password"),

    )
sealed interface LoginRequest : JsonDocument {
    data class PasswordLoginRequest(
        @JsonAlias("user", "username", "subj", "u")
        val email: String,
        @JsonAlias("pass", "password", "pw", "pwd", "p")
        val password: String
    ) : LoginRequest

}