package de.phyrone.kiwi.proto

import de.phyrone.kiwi.snowflake.SnowflakeServiceGrpcKt
import io.grpc.ChannelCredentials
import io.grpc.Grpc
import io.grpc.InsecureChannelCredentials

fun createSnowflakeClient(
    address: String,
    credentials: ChannelCredentials? = null
): SnowflakeServiceGrpcKt.SnowflakeServiceCoroutineStub {
    val channel = Grpc.newChannelBuilder(address, credentials ?: InsecureChannelCredentials.create())
        .enableRetry()
        .build()
    return SnowflakeServiceGrpcKt.SnowflakeServiceCoroutineStub(channel)
}