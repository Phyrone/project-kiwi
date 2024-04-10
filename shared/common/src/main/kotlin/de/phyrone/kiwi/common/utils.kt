package de.phyrone.kiwi.common

import com.google.common.flogger.FluentLogger
import com.google.common.flogger.LazyArg
import com.google.common.flogger.LazyArgs
import com.google.common.flogger.backend.LoggerBackend
import com.google.common.flogger.backend.Platform
import org.slf4j.LoggerFactory
import java.io.File
import java.util.*
import kotlin.math.absoluteValue
import kotlin.math.ln
import kotlin.math.pow
import ch.qos.logback.classic.Level as LogbackLevel
import ch.qos.logback.classic.Logger as LogbackLogger
import org.slf4j.event.Level as SLF4JLevel

@Suppress("NOTHING_TO_INLINE")
inline fun logger() = FluentLogger.forEnclosingClass()

fun loggerFor(clazz: Class<*>) = initLogger(Platform.getBackend(clazz.name))
inline fun <reified T> loggerFor() = loggerFor(T::class.java)

private val floggerConstructor by lazy {
    FluentLogger::class.java.getDeclaredConstructor(LoggerBackend::class.java).also {
        it.isAccessible = true
    }
}

private fun initLogger(loggerBackend: LoggerBackend) = floggerConstructor.newInstance(loggerBackend)

fun setLoggerLevel(
    level: SLF4JLevel
) {
    (LoggerFactory.getLogger(LogbackLogger.ROOT_LOGGER_NAME) as LogbackLogger).level =
        LogbackLevel.convertAnSLF4JLevel(level)
}

fun shutdownLogger() {

    val loggerContext = (LoggerFactory.getILoggerFactory() as ch.qos.logback.classic.LoggerContext)
    loggerContext.stop()
}

@Suppress("NOTHING_TO_INLINE")
inline fun <T> lazyArg(lambdaOrMethodReference: LazyArg<T>) = LazyArgs.lazy(lambdaOrMethodReference)

val HUMAN_MEMORY_SI_PREFIXES = arrayOf("B", "KB", "MB", "GB", "TB", "PB", "EB")
val HUMAN_MEMORY_IEC_PREFIXES = arrayOf("B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB")

/**
 * Humanize a memory size to make it more readable
 * @param size the size in bytes which should be humanized
 * @param decimals the amount of decimals which should be used after the comma (default: 2)
 * @param si if the SI unit should be used if set to false the IEC unit will be used (default: true)
 *   look at this [Wikipedia Article](https://en.wikipedia.org/wiki/Binary_prefix) for more information
 * @param unsigned if set to true negative values is mapped to its unsigned counterpart (default: false)
 */
fun humanizeMemorySize(
    size: Long,
    decimals: Int = 2,
    si: Boolean = true,
    unsigned: Boolean = false,
    locale: Locale= Locale.getDefault(),
): String {
    if (size == 0L) return "0 B"

    val unit = if (si) 1000 else 1024
    val prefixes = if (si) HUMAN_MEMORY_SI_PREFIXES else HUMAN_MEMORY_IEC_PREFIXES
    val isNegative = size < 0
    val bytes = when {
        unsigned -> size.toULong().toDouble()
        isNegative -> size.toDouble().absoluteValue
        else -> size.toDouble()
    }
    val exponent = (ln(bytes) / ln(unit.toDouble())).toInt()
    val value = bytes / unit.toDouble().pow(exponent)
    val value2 = if (isNegative) -value else value
    val formattedValue = "%.${decimals}f".format(locale,value2)
    return "$formattedValue ${prefixes[exponent]}"
}

/// Env File is must be at most 16MB
const val MAX_ENV_FILE_SIZE = 1024 * 1024 * 16

@JvmOverloads
fun readEnvFile(
    file: File = File(".env")
): Map<String, String>? {
    if (file.exists() && file.isFile) {
        if (!file.canRead()) {
            return null
        }
        if (file.length() > MAX_ENV_FILE_SIZE) {
            logger().atWarning()
                .log(
                    "file '%1' is too big! (%1) and will not be read!",
                    file.absolutePath,
                    humanizeMemorySize(file.length(), si = false)
                )
            return null
        }

        return file.readLines()
            .asSequence()
            .filter { it.isNotBlank() }
            .map { it.trimStart() }
            .filterNot { it.startsWith("#") }
            .map { it.split("=", limit = 2) }
            .filter { it.size == 2 }
            .filter { it[0].isNotBlank() }
            .associate { (key, value) -> key to value }

    }
    return null

}