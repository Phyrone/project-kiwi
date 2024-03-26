package de.phyrone.kiwi.gateway.packets

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import de.phyrone.kiwi.gateway.documents.JsonDocument


@JsonTypeInfo(
    use = JsonTypeInfo.Id.NAME,
    include = JsonTypeInfo.As.PROPERTY,
    property = "t",
)
sealed interface SubscriptionTopic : JsonDocument

@JsonTypeName("m")
data class ChannelMessages(
    val channel: String,
    val limit: Int?,
) : SubscriptionTopic