package de.phyrone.kiwi.gateway

import com.google.common.flogger.FluentLogger
import de.phyrone.kiwi.common.ShutdownHandler
import de.phyrone.kiwi.common.Slf4jKoinLogger
import de.phyrone.kiwi.common.config.ConfigStartupParams
import de.phyrone.kiwi.common.executeOrdered
import de.phyrone.kiwi.common.setLoggerLevel
import de.phyrone.kiwi.common.systems.StartupRunner
import de.phyrone.kiwi.database.DatabaseStartupParams
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import org.koin.core.Koin
import org.koin.core.KoinApplication
import org.koin.core.context.startKoin
import org.koin.core.extension.coroutinesEngine
import org.koin.dsl.bind
import org.koin.dsl.binds
import org.koin.dsl.module
import org.koin.ksp.generated.module
import picocli.CommandLine
import java.lang.management.ManagementFactory
import kotlin.system.exitProcess
import kotlin.time.Duration.Companion.milliseconds
import kotlin.time.measureTimedValue


object Main {

    private val logger = FluentLogger.forEnclosingClass()

    @JvmStatic
    fun main(args: Array<String>) {
        val commandLine = CommandLine(StartupParams::class.java)
        exitProcess(commandLine.execute(*args))
    }


    fun runApplication(params: StartupParams) {
        setLoggerLevel(params.logLevel)

        logger.atInfo().log("Starting Gateway...")
        val (koinApplication, koinStartupDuration) = measureTimedValue {
            startKoin {
                coroutinesEngine()
                logger(Slf4jKoinLogger)
                modules(
                    ModuleBundle.module,
                    module(true) {
                        single { params } binds arrayOf(
                            ConfigStartupParams::class, ConfigStartupParams::class, DatabaseStartupParams::class
                        )
                        single { this@startKoin } bind KoinApplication::class
                        single { get<KoinApplication>().koin } bind Koin::class
                    },
                )
            }
        }
        koinApplication.createEagerInstances()
        logger.atInfo().log("Koin started %s", koinStartupDuration)
        val koin = koinApplication.koin
        CoroutineScope(MainLoop).launch {
            runApplicationPostLoop(koin)
        }
        ShutdownHandler(koin).register()
        MainLoop.run_loop()
    }

    private suspend fun runApplicationPostLoop(koin: Koin) {
        koin.getAll<StartupRunner>().executeOrdered { it.onStartup() }
        //assuming that the jvm started just before the startup
        val uptime = ManagementFactory.getRuntimeMXBean().uptime.milliseconds
        logger.atInfo().log("Startup finished in %s", uptime)
    }
}