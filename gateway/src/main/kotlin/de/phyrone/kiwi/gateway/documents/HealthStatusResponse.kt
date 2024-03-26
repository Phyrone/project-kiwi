package de.phyrone.kiwi.gateway.documents

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.dataformat.xml.annotation.JacksonXmlRootElement

@JacksonXmlRootElement(localName = "health-status")
data class HealthStatusResponse(
    @JsonProperty("healthy")
    val isHealthy: Boolean,
)
