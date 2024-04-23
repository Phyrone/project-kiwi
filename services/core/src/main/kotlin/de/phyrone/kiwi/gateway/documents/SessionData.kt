package de.phyrone.kiwi.gateway.documents

import com.auth0.jwt.RegisteredClaims
import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonInclude
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.core.JsonGenerator
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.JsonDeserializer
import com.fasterxml.jackson.databind.JsonSerializer
import com.fasterxml.jackson.databind.SerializerProvider
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import java.time.Instant


@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonIgnoreProperties(ignoreUnknown = true)
data class SessionData(
    @JsonProperty(RegisteredClaims.SUBJECT)
    val account: Long,
    @JsonProperty(RegisteredClaims.EXPIRES_AT)
    @JsonSerialize(using = UnixTimestampSerializer::class)
    @JsonDeserialize(using = UnixTimestampDeserializer::class)
    val expires: Instant? = null,
) {

    private class UnixTimestampSerializer : JsonSerializer<Instant>() {
        override fun serialize(value: Instant, gen: JsonGenerator, serializers: SerializerProvider?) {
            gen.writeNumber(value.epochSecond)
        }
    }

    private class UnixTimestampDeserializer : JsonDeserializer<Instant>() {
        override fun deserialize(parser: JsonParser, ctxt: DeserializationContext?): Instant {
            return Instant.ofEpochSecond(parser.valueAsLong)
        }
    }


}