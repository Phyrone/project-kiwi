package de.phyrone.kiwi.permission

import io.r2dbc.postgresql.PostgresqlConnectionFactory
import org.koin.core.annotation.Singleton

@Singleton
class ProfilePermissionManager(
    private val postgres : PostgresqlConnectionFactory
) {


}