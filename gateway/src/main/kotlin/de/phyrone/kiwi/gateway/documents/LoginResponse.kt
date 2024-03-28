package de.phyrone.kiwi.gateway.documents

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.dataformat.xml.annotation.JacksonXmlRootElement
import de.phyrone.kiwi.gateway.ExtendedRFC9457Error
import io.ktor.http.HttpStatusCode

@JacksonXmlRootElement(localName = "login-response")
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "outcome")
interface LoginResponse : JsonDocument {
    val success: Boolean

    @JsonTypeName("success")
    data class Success(
        @JsonProperty("account-id")
        val accountID: Long,
        val token: String,
    ) : LoginResponse {
        override val success: Boolean = true
    }

    @JsonTypeName("invalid-credentials")
    data object InvalidCredentials : LoginResponse, ExtendedRFC9457Error(
        HttpStatusCode.Unauthorized,
        "TODO: invalid-credentials",
        "Invalid credentials",
        "The provided credentials are invalid",
    )
}