package de.phyrone.kiwi.database

import de.phyrone.kiwi.common.systems.ShutdownHook
import io.r2dbc.postgresql.PostgresqlConnectionFactory
import org.koin.core.annotation.Named
import org.koin.core.annotation.Single
import javax.sql.DataSource


@Single(binds = [ShutdownHook::class,DatabaseShutdown::class])
@Named("DatabaseShutdown")
class DatabaseShutdown(
    private val dataSource: DataSource,
) : ShutdownHook {
    override suspend fun onShutdown() {
        (dataSource as? AutoCloseable)?.close()
    }
}