package de.phyrone.kiwi.gateway

import com.typesafe.config.Config
import io.minio.GetPresignedObjectUrlArgs
import io.minio.MinioAsyncClient
import io.minio.SetBucketPolicyArgs
import io.minio.SetBucketTagsArgs
import io.minio.http.Method
import java.util.concurrent.*


fun minio_client(
    config: Config
): MinioAsyncClient {

    return MinioAsyncClient.builder()
        .endpoint(config.getString("minio.endpoint"))
        .build()
}

suspend fun createTempToken(
    client: MinioAsyncClient
) {
    client.getPresignedObjectUrl(
        GetPresignedObjectUrlArgs
            .builder()
            .method(Method.GET)
            .expiry(6,TimeUnit.HOURS)
            .`object`("")
            .build()
    )

}