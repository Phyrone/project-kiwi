package de.phyrone.kiwi.gateway

import de.phyrone.kiwi.proto.createSnowflakeClient
import org.koin.core.annotation.Single


@Single
fun snowflakeClient() = createSnowflakeClient(
    "localhost:50152"
)