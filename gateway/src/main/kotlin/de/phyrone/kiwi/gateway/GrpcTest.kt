package de.phyrone.kiwi.gateway

import de.phyrone.kiwi.snowflake.SnowflakeRequest
import de.phyrone.kiwi.snowflake.SnowflakeServiceGrpcKt
import io.grpc.Grpc
import io.grpc.InsecureChannelCredentials
import io.grpc.ManagedChannelBuilder
import kotlinx.coroutines.runBlocking
import java.util.concurrent.TimeUnit

object GrpcTest {

    @JvmStatic
    fun main(args: Array<String>): Unit = runBlocking {

        val client = Grpc.newChannelBuilder("localhost:2322", InsecureChannelCredentials.create())
            .enableRetry()
            .build()

        val snowflakeService = SnowflakeServiceGrpcKt.SnowflakeServiceCoroutineStub(client)
        val response = snowflakeService.getSnowflakes(SnowflakeRequest.newBuilder().setCount(12).build())
        client.shutdown()
        client.awaitTermination(10, TimeUnit.SECONDS)
        println("Snowflakes:")
        response.snowflakesList.forEach {
            println(it)
        }

    }
}