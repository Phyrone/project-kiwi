package de.phyrone.kiwi.gateway

import de.phyrone.kiwi.common.config.ConfigStartupParams
import de.phyrone.kiwi.database.DatabaseStartupParams
import picocli.CommandLine.Command
import picocli.CommandLine.Option
import org.slf4j.event.Level as SLF4JLevel

@Command()
class StartupParams : Runnable, ConfigStartupParams, DatabaseStartupParams {

    @Option(
        names = ["-b", "--bind"],
        description = ["The address to bind to"],
        required = true,
        defaultValue = "0.0.0.0:7080",
    )
    lateinit var binds: List<String>

    @Option(
        names = ["-f", "--file"],
        description = ["The config file to use"],
        required = true,
        defaultValue = "application.conf",
    )
    override lateinit var file: String

    @Option(
        names = ["--danger--fresh-database"],
        description = [
            "If set the database will be wiped and recreated",
            "",
            "WARNING: This will delete ALL data in the database",
            "it drops ALL TABLES (including unrelated) in the selected database and recreates them based on the schema",
            "",
            "it is intended for development and should not be used in production for the mentioned reason!",
        ],
        required = false,
        defaultValue = "false",
    )
    override var fresh: Boolean = false

    @Option(
        names = ["--log-level", "-l"],
        description = ["The log level to use"],
        required = false,
        defaultValue = "INFO",
    )
    var logLevel: SLF4JLevel = SLF4JLevel.INFO

    override fun run() = Main.runApplication(this)

}