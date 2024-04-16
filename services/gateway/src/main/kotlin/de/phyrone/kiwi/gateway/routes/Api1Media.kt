package de.phyrone.kiwi.gateway.routes

import de.phyrone.kiwi.gateway.WebApplication
import io.ktor.http.HttpStatusCode
import io.ktor.server.application.Application
import io.ktor.server.application.call
import io.ktor.server.auth.authenticate
import io.ktor.server.request.receiveStream
import io.ktor.server.response.respond
import io.ktor.server.routing.put
import io.ktor.server.routing.route
import io.ktor.server.routing.routing
import io.minio.ComposeObjectArgs
import io.minio.GetObjectArgs
import io.minio.GetPresignedObjectUrlArgs
import io.minio.MinioAsyncClient
import io.minio.PutObjectArgs
import io.minio.SetBucketPolicyArgs
import io.minio.SetObjectRetentionArgs
import io.minio.UploadObjectArgs
import io.minio.http.Method
import kotlinx.coroutines.future.await
import org.koin.core.annotation.Single

@Single
class Api1Media(
    private val minioClient: MinioAsyncClient
) : WebApplication {


    override fun Application.apply() {
        routing {
       
            route("$API_V1_BASE_PATH/media") {
                authenticate(optional = true) {
                    //upload media
                    put("{id}") {
                        val mediaId = call.parameters["id"] ?: error("no id provided")
                        minioClient.getPresignedObjectUrl(GetPresignedObjectUrlArgs.builder()
                            .`object`("")
                            .build()
                        )

                        call.receiveStream().use { upload ->
                            minioClient.putObject(
                                PutObjectArgs.builder()
                                    .stream(upload, -1, 16 * 1024 * 1024)
                                    .build()
                            ).await()
                            minioClient.uploadObject(UploadObjectArgs.builder().build())
                        }


                        call.respond(HttpStatusCode.NotImplemented, mediaId)
                    }
                }
            }
        }
    }
}