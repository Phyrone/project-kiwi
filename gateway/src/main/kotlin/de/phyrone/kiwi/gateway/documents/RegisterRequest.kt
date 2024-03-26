package de.phyrone.kiwi.gateway.documents

import com.fasterxml.jackson.annotation.JsonAlias
import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.dataformat.xml.annotation.JacksonXmlRootElement

@JacksonXmlRootElement(localName = "register-request")
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "with")
@JsonSubTypes(
    JsonSubTypes.Type(value = RegisterRequest.PasswordRegisterRequest::class, name = "password"),
)
interface RegisterRequest : JsonDocument {

    data class PasswordRegisterRequest(
        @JsonAlias("email")
        val user: String,
        val password: String
    ) : RegisterRequest
}