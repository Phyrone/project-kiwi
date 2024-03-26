package de.phyrone.kiwi.gateway.packets

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import de.phyrone.kiwi.gateway.documents.JsonDocument

@JsonTypeInfo(
    use = JsonTypeInfo.Id.NAME,
    include = JsonTypeInfo.As.PROPERTY,
    property = "o",
)
sealed interface WsPacketClientbound : JsonDocument

@JsonTypeName("i")
data class PacketHello(
    @JsonProperty("t")
    val token: String,
    @JsonProperty("v")
    val heartbeatInterval: Int?,
) : WsPacketClientbound

@JsonTypeName("r")
data class PacketHeartbeat(
    @JsonProperty("d")
    val sequence: Int,
) : WsPacketClientbound

@JsonTypeName("s")
data class PacektSubsrcibe(
    @JsonProperty("t")
    val topic: SubscriptionTopic,
) : WsPacketClientbound

@JsonTypeName("u")
data class PacketUnsubscribe(
    @JsonProperty("t")
    val topic: SubscriptionTopic,
) : WsPacketClientbound