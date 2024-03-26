package de.phyrone.kiwi.database

import java.net.URI


//TODO: add support for tls
data class PostgresConfiguration(
    val hosts: List<String>,
    val database: String,
    val username: String? = null,
    val password: String? = null,

    ) {
    val hostPortPairs = hosts.map {
        val uri = URI("postgres://$it")
        Pair(uri.host, uri.port.takeIf { it != -1 })
    }

}
