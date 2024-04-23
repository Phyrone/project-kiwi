package de.phyrone.kiwi.gateway.documents

import com.fasterxml.jackson.annotation.JsonAlias
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.dataformat.xml.annotation.JacksonXmlRootElement

@JacksonXmlRootElement(localName = "register-request")
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "with")
interface RegisterRequest : JsonDocument {

    @JsonTypeName("password")
    data class PasswordRegisterRequest(
        @JsonAlias("email", "user", "username", "subj", "u")
        val user: String,
        @JsonAlias("pass", "password", "pw", "pwd", "p")
        val password: String
    ) : RegisterRequest
}