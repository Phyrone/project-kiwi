package de.phyrone.kiwi.gateway.documents

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo
import de.phyrone.kiwi.gateway.ExtendedRFC9457Error
import io.ktor.http.HttpStatusCode
import com.fasterxml.jackson.annotation.JsonSubTypes.Type as SubType

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "outcome")
@JsonSubTypes(
    SubType(value = LoginResponse.Success::class, name = "success"),
    SubType(value = LoginResponse.InvalidCredentials::class, name = "invalid-credentials")
)
interface LoginResponse : JsonDocument {
    val success: Boolean

    data class Success(
        @JsonProperty("account-id")
        val accountID: Long,
        val token: String,
    ) : LoginResponse{
        override val success: Boolean = true
    }

    data object InvalidCredentials: LoginResponse, ExtendedRFC9457Error(
        HttpStatusCode.Unauthorized,
        "TODO: invalid-credentials",
        "Invalid credentials",
        "The provided credentials are invalid",
    )
}