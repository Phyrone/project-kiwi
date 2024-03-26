package de.phyrone.kiwi.database

import com.typesafe.config.Config
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import de.phyrone.kiwi.common.CommonModule
import de.phyrone.kiwi.common.systems.ShutdownHook
import de.phyrone.kiwi.common.systems.StartupRunner
import io.r2dbc.postgresql.PostgresqlConnectionConfiguration
import io.r2dbc.postgresql.PostgresqlConnectionFactory
import org.flywaydb.core.Flyway
import org.koin.core.annotation.Module
import org.koin.core.annotation.Single
import javax.sql.DataSource

@Module(includes = [CommonModule::class])
class DatabaseModule {

    @Single(binds = [ShutdownHook::class])
    fun databaseShutdown(
        dataSource: DataSource,
    ): DatabaseShutdown = DatabaseShutdown(dataSource)

    @Single(binds = [DatabaseMigrator::class, StartupRunner::class])
    fun migrator(
        flyway: Flyway,
        startupParams: DatabaseStartupParams
    ): DatabaseMigrator = DatabaseMigrator(startupParams, flyway)

    @Single
    fun postgresConfiguration(
        config: Config
    ): PostgresConfiguration = PostgresConfiguration(
        config.getStringList("database.hosts"),
        config.getString("database.database"),
        config.getString("database.username"),
        config.getString("database.password")
    )


    @Single
    fun postgres(
        configuration: PostgresConfiguration
    ): PostgresqlConnectionFactory {
        require(configuration.hosts.isNotEmpty()) { "At least one host is required" }
        val builder = PostgresqlConnectionConfiguration.builder()
        for ((host, port) in configuration.hostPortPairs) {
            port?.let { notNullPort -> builder.addHost(host, notNullPort) } ?: builder.addHost(host)
        }
        configuration.database.let(builder::database)
        configuration.username?.let(builder::username)
        configuration.password?.let(builder::password)

        return PostgresqlConnectionFactory(builder.build()).also { factory ->
            factory.create().block()?.close()?.block()
        }
    }

    @Single
    fun flyway(
        dataSource: DataSource,
        params: DatabaseStartupParams
    ): Flyway {
        return Flyway.configure()
            .dataSource(dataSource)
            .cleanDisabled(!params.fresh)
            .batch(true)
            .locations("classpath:files/db/migration")
            .load()
    }

    @Single
    fun hikariConfig(
        configuration: PostgresConfiguration
    ): HikariConfig {
        val hikariConfig = HikariConfig()
        require(configuration.hosts.isNotEmpty()) { "At least one host is required" }
        val jdbcURL = buildString {
            append("jdbc:postgresql://")
            append(configuration.hosts.joinToString(separator = ","))
            append("/${configuration.database}")
        }
        hikariConfig.driverClassName = POSTGRES_DRIVER
        hikariConfig.jdbcUrl = jdbcURL

        configuration.username?.let(hikariConfig::setUsername)
        configuration.password?.let(hikariConfig::setPassword)

        //since we primarily use r2dbc we don't need a pool to be active all the time
        hikariConfig.minimumIdle = 0
        //but in case we need to do a lot of work we can have a lot of connections
        hikariConfig.maximumPoolSize = 15

        hikariConfig.isReadOnly = false
        hikariConfig.isAllowPoolSuspension = true

        hikariConfig.validate()
        return hikariConfig
    }

    @Single(binds = [HikariDataSource::class, DataSource::class])
    fun datasouce(
        hikariConfig: HikariConfig
    ): HikariDataSource {
        return HikariDataSource(hikariConfig)
    }

    companion object {
        const val POSTGRES_DRIVER = "org.postgresql.Driver"
    }
}