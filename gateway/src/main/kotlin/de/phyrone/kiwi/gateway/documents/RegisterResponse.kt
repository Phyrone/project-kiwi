package de.phyrone.kiwi.gateway.documents

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.dataformat.xml.annotation.JacksonXmlRootElement
import de.phyrone.kiwi.gateway.RFC9457Error
import io.ktor.http.HttpStatusCode

@JacksonXmlRootElement(localName = "login-response")
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "outcome")
@JsonSubTypes(
    JsonSubTypes.Type(value = RegisterResponse.Success::class, name = "success"),
    JsonSubTypes.Type(value = RegisterResponse.UserAlreadyExists::class, name = "user-already-exists")
)
sealed interface RegisterResponse {
    @get:JsonProperty("success")
    val success: Boolean

    data class Success(
        @JsonProperty("account-id")
        val accountID: Long,
        val user: String,
        val token: String,
        override val success: Boolean = true
    ) : RegisterResponse

    data class UserAlreadyExists(
        val user: String,
        override val success: Boolean = false
    ) : RegisterResponse, RFC9457Error(
        status = HttpStatusCode.Conflict,
        title = "User already exists",
        type = "TODO:user-already-exists",
        instance = "TODO:instance",
        detail = "The user you want to register already exists. You can try to login or reset your password."
    )
}