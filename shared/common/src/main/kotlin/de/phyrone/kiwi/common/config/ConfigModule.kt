package de.phyrone.kiwi.common.config

import com.typesafe.config.Config
import com.typesafe.config.ConfigFactory
import com.typesafe.config.ConfigParseOptions
import de.phyrone.kiwi.common.logger
import org.koin.core.annotation.Module
import org.koin.core.annotation.Single
import java.io.File

@Module(includes = [])
class ConfigModule {


    //TODO determine file from params
    @Single(createdAtStart = true)
    fun config(
        params: ConfigStartupParams
    ): Config {
        val defaultConfig = ConfigFactory.parseResources(
            "application.conf", ConfigParseOptions.defaults()
                .setAllowMissing(true)
        )
        val configFile = File(params.file)
        val fileConfig = if (configFile.exists()) {
            ConfigFactory.parseFile(
                configFile,
                ConfigParseOptions.defaults()
                    .setAllowMissing(false)
            )
        } else {
            ConfigFactory.parseFileAnySyntax(
                configFile,
                ConfigParseOptions.defaults().setAllowMissing(true)
            )
        }
        val envOverride = ConfigFactory.systemEnvironmentOverrides()
        val env = ConfigFactory.systemEnvironment()
        return envOverride
            .withFallback(env)
            .withFallback(fileConfig)
            .withFallback(defaultConfig)
    }

    companion object {
        private val logger = logger()
    }

}