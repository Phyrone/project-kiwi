package de.phyrone.kiwi.database

import de.phyrone.kiwi.common.systems.ShutdownHook
import io.r2dbc.postgresql.PostgresqlConnectionFactory
import javax.sql.DataSource


class DatabaseShutdown(
    private val dataSource: DataSource,
) : ShutdownHook {
    override suspend fun onShutdown() {
        (dataSource as? AutoCloseable)?.close()
    }
}