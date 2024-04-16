package de.phyrone.kiwi.gateway

import com.typesafe.config.Config
import io.minio.MinioAsyncClient
import io.minio.MinioClient


fun minio_client(
    config:Config
):MinioAsyncClient{

    return MinioAsyncClient.builder()
        .endpoint(config.getString("minio.endpoint"))
        .build()
}