package de.phyrone.kiwi.common

import com.google.common.flogger.FluentLogger
import com.google.common.flogger.LazyArg
import com.google.common.flogger.LazyArgs
import com.google.common.flogger.backend.LoggerBackend
import com.google.common.flogger.backend.Platform
import org.slf4j.LoggerFactory
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