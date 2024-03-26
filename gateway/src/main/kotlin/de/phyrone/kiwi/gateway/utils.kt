package de.phyrone.kiwi.gateway

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.xml.XmlMapper
import com.fasterxml.jackson.module.kotlin.KotlinFeature
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import io.ktor.http.ContentType
import io.ktor.serialization.Configuration
import io.ktor.serialization.jackson.JacksonConverter
import io.ktor.server.application.ApplicationCall
import io.ktor.server.request.accept
import io.ktor.server.request.contentType
import io.ktor.server.response.respondOutputStream
import org.koin.mp.KoinPlatform
import kotlin.time.Duration
import kotlin.time.DurationUnit


private val socketAddressRegex = "([\\w.\\-]+)?:(\\d+)".toRegex()


public fun Configuration.jackson(
    contentType: ContentType = ContentType.Application.Json,
    streamRequestBody: Boolean = true,
    objectMapper: ObjectMapper
) {
    val converter = JacksonConverter(objectMapper, streamRequestBody)
    register(contentType, converter)
}

public fun <T : ObjectMapper> T.setup(): T {
    registerKotlinModule {
        enable(KotlinFeature.NullToEmptyMap)
        enable(KotlinFeature.NullToEmptyCollection)
    }
    findAndRegisterModules()

    return this
}

public fun ApplicationCall.acceptsContent(): List<ContentType> {
    val accept = request.accept()
    val contentType = request.accept()
        ?.split(',')
        ?.map { it.trim() }
        ?.map { ContentType.parse(it) }
        ?: listOf(ContentType.Any)
    return contentType
}

fun ContentType.anyXml() =
    this == ContentType.Application.Xml || this == ContentType.Text.Xml || this == ContentType.Application.ProblemXml

fun ContentType.anyJson() =
    this == ContentType.Application.Json || this == ContentType.Application.HalJson || this == ContentType.Application.ProblemJson

public suspend fun ApplicationCall.respondError(
    error: RFC9457Error
) {
    //when client accepts problem+xml or sends xml but does not accept json or problem+json then respond with xml
    //when accepts json or problem+json then respond with problem+json
    val sendContentType = request.contentType()

    val accepts = acceptsContent()
    val respondType = when {
        (accepts.any { it.anyXml() } || sendContentType.match(ContentType.Application.Xml))
                && !accepts.any { it.anyJson() }
        -> ContentType.Application.ProblemXml

        else -> ContentType.Application.ProblemJson
    }

    respondOutputStream(respondType, error.status) {
        val koin = KoinPlatform.getKoin()
        val mapper = when (respondType) {
            ContentType.Application.ProblemXml -> koin.get<XmlMapper>()
            else -> koin.get<ObjectMapper>()
        }
        mapper.writeValue(this, error)
    }
}

const val SERVER_TIMING = "Server-Timing"

fun ApplicationCall.addTiming(
    name: String,
    duration: Duration? = null
) {
    val timingData = buildString {
        append(name)
        if (duration != null) {
            append(";dur=${duration.toDouble(DurationUnit.MILLISECONDS)}")
        }

    }
    response.headers.append(SERVER_TIMING, timingData)
}