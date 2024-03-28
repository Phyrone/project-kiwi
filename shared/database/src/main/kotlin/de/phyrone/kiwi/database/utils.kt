package de.phyrone.kiwi.database

import io.r2dbc.postgresql.PostgresqlConnectionFactory
import io.r2dbc.postgresql.api.PostgresqlConnection
import kotlinx.coroutines.reactive.awaitFirstOrNull
import kotlinx.coroutines.reactor.awaitSingleOrNull
import org.jetbrains.exposed.sql.Database
import kotlin.contracts.ExperimentalContracts
import kotlin.contracts.contract

@OptIn(ExperimentalContracts::class)
suspend inline fun <T> PostgresqlConnectionFactory.connection(
    work: (connection: PostgresqlConnection) -> T
): T {
    contract {
        callsInPlace(work, kotlin.contracts.InvocationKind.EXACTLY_ONCE)
    }

    val connection = this.create().awaitFirstOrNull() ?: error("could not get a postgres connection")
    try {
        return work(connection)
    } finally {
        connection.close()
            .awaitSingleOrNull()
    }
}

fun a(){
    Database.connect("")
}