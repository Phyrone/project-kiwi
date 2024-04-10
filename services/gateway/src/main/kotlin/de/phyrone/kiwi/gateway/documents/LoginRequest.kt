package de.phyrone.kiwi.gateway.documents

import com.fasterxml.jackson.annotation.JsonAlias
import com.fasterxml.jackson.annotation.JsonIgnoreType
import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "with")
@JsonSubTypes(
    JsonSubTypes.Type(value = LoginRequest.PasswordLoginRequest::class, name = "password"),

    )
sealed interface LoginRequest : JsonDocument {

    @JsonTypeName("password")
    data class PasswordLoginRequest(
        @JsonAlias("user", "username", "subj", "u")
        val email: String,
        @JsonAlias("pass", "password", "pw", "pwd", "p")
        val password: String
    ) : LoginRequest

}