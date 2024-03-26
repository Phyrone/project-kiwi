@file:Suppress("unused")

package de.phyrone.kiwi.gateway

import com.fasterxml.jackson.annotation.JsonInclude
import com.fasterxml.jackson.core.JsonGenerator
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.JsonDeserializer
import com.fasterxml.jackson.databind.JsonSerializer
import com.fasterxml.jackson.databind.SerializerProvider
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import io.ktor.http.HttpStatusCode


@JsonInclude(JsonInclude.Include.NON_NULL, content = JsonInclude.Include.NON_NULL)
open class RFC9457Error(
    @field:JsonSerialize(using = StatusCodeSerializer::class)
    @JsonDeserialize(using = StatusCodeDeserializer::class)
    val status: HttpStatusCode,
    val type: String,
    val title: String,
    val detail: String,
    val instance: String? = null,
) {
    private class StatusCodeSerializer : JsonSerializer<HttpStatusCode>() {
        override fun serialize(value: HttpStatusCode, gen: JsonGenerator, serializers: SerializerProvider) {
            gen.writeNumber(value.value)
        }
    }

    private class StatusCodeDeserializer : JsonDeserializer<HttpStatusCode>() {

        override fun deserialize(p: JsonParser, ctxt: DeserializationContext): HttpStatusCode {
            return HttpStatusCode.fromValue(p.intValue)
        }
    }
}


open class ExtendedRFC9457Error(
    status: HttpStatusCode,
    type: String,
    title: String,
    detail: String,
    instance: String? = null,
    val success: Boolean = false,
) : RFC9457Error(status, type, title, detail, instance)

open class RFC9457ResourceNotFound(
    route: String,
) : ExtendedRFC9457Error(
    status = HttpStatusCode.NotFound,
    type = "https://httpstatuses.com/404",
    title = "Not Found",
    detail = "The requested resource could not be found.",
    instance = route,
)