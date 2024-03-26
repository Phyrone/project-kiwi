package de.phyrone.kiwi.database

import de.phyrone.kiwi.common.logger
import de.phyrone.kiwi.common.systems.StartupRunner
import org.flywaydb.core.Flyway
import kotlin.time.measureTime

class DatabaseMigrator(
    private val params: DatabaseStartupParams,
    private val flyway: Flyway
) : StartupRunner {

    fun up() {
        logger.atInfo().log("Checking for database migration and applying if necessary.")
        measureTime {
            flyway.migrate()
        }.also { time ->
            logger.atInfo().log("Database migration done in %s", time)
        }
    }

    fun fresh() {
        logger.atWarning().log("Doing a fresh database migration. This will delete all data in the database.")
        measureTime {
            flyway.clean()
            flyway.migrate()
        }.also { time ->
            logger.atInfo().log("Database refresh done in %s", time)
        }

    }

    override suspend fun onStartup() {

        if (params.fresh) {
            fresh()
        } else {
            up()
        }

    }

    companion object {
        private val logger = logger()
    }
}