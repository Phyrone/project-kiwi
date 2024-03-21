package de.phyrone.kiwi.gateway.packets

import com.fasterxml.jackson.annotation.JsonAutoDetect
import com.fasterxml.jackson.annotation.JsonFormat
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName

@JsonTypeInfo(
    use = JsonTypeInfo.Id.NAME,
    include = JsonTypeInfo.As.PROPERTY,
    property = "o",
)
interface WsPacketServerbound : JsonDocument

@JsonTypeName("c")
data class WsPacketServerboundMessagePush(
    @JsonProperty("c") val channel: ULong,
    @JsonProperty("i") val messageId: ULong,
    @JsonProperty("p") val payload: Any,
) : WsPacketServerbound

@JsonTypeName("r")
data class WsPacketServerboundMessageUpdate(
    @JsonProperty("c") val channel: ULong,
    @JsonProperty("i") val messageId: ULong,
    @JsonProperty("p") val payload: Any?,
) : WsPacketServerbound



