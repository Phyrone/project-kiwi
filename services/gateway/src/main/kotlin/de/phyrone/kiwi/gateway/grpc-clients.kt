package de.phyrone.kiwi.gateway

import com.typesafe.config.Config
import de.phyrone.kiwi.proto.createSnowflakeClient
import de.phyrone.kiwi.snowflake.SnowflakeServiceGrpcKt
import io.grpc.ChannelCredentials
import io.grpc.InsecureChannelCredentials
import org.koin.core.annotation.Single


@Single
fun snowflakeClient(
    config: Config
): SnowflakeServiceGrpcKt.SnowflakeServiceCoroutineStub {
InsecureChannelCredentials.create()
    return createSnowflakeClient(
        address = config.getString("services.snowflake.address")
    )
}