plugins {
    kotlin("jvm")
    kotlin("kapt")
    id("com.google.cloud.tools.jib")
    id("com.google.devtools.ksp")
    application

}

group = parent?.group ?: error("parent not specified")
version = parent?.version ?: error("parent not specified")

repositories {
    mavenCentral()
}


dependencies {

    /* Shared Components */
    implementation(project(":shared:common"))
    implementation(project(":shared:proto"))
    implementation(project(":shared:database"))

    /* Kotlin */
    implementation(kotlin("stdlib-jdk8"))
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1-Beta")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-jdk8:1.8.1-Beta")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-reactor:1.8.1-Beta")

    /* Vert.x */
    //implementation(platform("io.vertx:vertx-stack-depchain:4.5.5"))
    //implementation("io.vertx:vertx-auth-jwt")
    //implementation("io.vertx:vertx-web")
    //implementation("io.vertx:vertx-grpc-server")
    //implementation("io.vertx:vertx-service-discovery")
    //implementation("io.vertx:vertx-json-schema")
    //implementation("io.vertx:vertx-stomp")
    //implementation("io.vertx:vertx-sockjs-service-proxy")
    //implementation("io.vertx:vertx-grpc-client")
    //implementation("io.vertx:vertx-config")
    //implementation("io.vertx:vertx-jdbc-client")
    //implementation("io.vertx:vertx-pg-client")
    //implementation("io.vertx:vertx-lang-kotlin-coroutines")
    //implementation("io.vertx:vertx-lang-kotlin")

    /* Ktor */
    implementation("io.ktor:ktor-server-netty-jvm:2.3.9")
    implementation("io.ktor:ktor-server-websockets-jvm:2.3.9")
    implementation("io.ktor:ktor-network-tls-certificates:2.3.10")
    implementation("io.ktor:ktor-server-auth:2.3.9")
    implementation("io.ktor:ktor-server-call-id:2.3.9")
    implementation("io.ktor:ktor-server-status-pages:2.3.9")
    implementation("io.ktor:ktor-server-content-negotiation:2.3.9")
    implementation("io.ktor:ktor-serialization-jackson:2.3.10")
    implementation("io.ktor:ktor-server-metrics-micrometer:2.3.9")
    implementation("io.ktor:ktor-server-rate-limit:2.3.9")
    implementation("io.ktor:ktor-server-request-validation:2.3.9")
    implementation("io.ktor:ktor-server-resources:2.3.9")
    implementation("io.ktor:ktor-server-locations:2.3.9")
    implementation("io.ktor:ktor-server-swagger:2.3.10")
    //implementation("io.ktor:ktor-server-openapi:2.3.9")
    implementation("io.ktor:ktor-server-auto-head-response:2.3.9")
    implementation("io.ktor:ktor-server-default-headers:2.3.9")
    implementation("io.ktor:ktor-server-compression:2.3.9")
    implementation("io.ktor:ktor-server-caching-headers:2.3.9")
    implementation("io.ktor:ktor-server-conditional-headers:2.3.9")
    implementation("io.ktor:ktor-server-cors:2.3.9")
    implementation("io.ktor:ktor-server-forwarded-header:2.3.9")
    implementation("io.ktor:ktor-server-rate-limit:2.3.9")

    /* Dependency Injection, EventBus, Modules etc. */
    implementation("io.insert-koin:koin-core:3.6.0-wasm-alpha2")
    implementation("io.insert-koin:koin-core-coroutines:3.6.0-wasm-alpha2")
    implementation("io.insert-koin:koin-annotations:1.3.1")
    ksp("io.insert-koin:koin-ksp-compiler:1.3.1")
    implementation("org.atteo.classindex:classindex:3.13")
    kapt("org.atteo.classindex:classindex:3.13")

    /* Database */
    implementation("com.rethinkdb:rethinkdb-driver:2.4.4")


    /* Other */
    implementation("com.auth0:java-jwt:4.4.0")
    implementation("info.picocli:picocli:4.7.5")
    kapt("info.picocli:picocli:4.7.5")
    //implementation("io.insert-koin:koin-ktor:3.6.0-wasm-alpha2")
    implementation("com.github.ben-manes.caffeine:caffeine:3.1.8")
    implementation("com.sksamuel.aedile:aedile-core:1.3.1")
    implementation("org.casbin:jcasbin:1.54.0")

    /* Json, MessagePack, etc. */
    implementation("com.fasterxml.jackson.core:jackson-core:2.17.0")
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.17.0")
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.17.0")
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jsr310:2.17.0")
    implementation("org.msgpack:jackson-dataformat-msgpack:0.9.8")
    implementation("com.fasterxml.jackson.dataformat:jackson-dataformat-xml:2.15.0")
    implementation("com.fasterxml.jackson.dataformat:jackson-dataformat-protobuf:2.17.0")
    implementation("com.fasterxml.jackson.dataformat:jackson-dataformat-cbor:2.17.0")
    implementation("com.fasterxml.jackson.dataformat:jackson-dataformat-yaml:2.17.0")
    implementation("com.fasterxml.jackson.dataformat:jackson-dataformat-csv:2.17.0")


}

kotlin {
    jvmToolchain(17)
}

application {
    mainClass.set("de.phyrone.kiwi.gateway.Main")
    val isDevelopment: Boolean = project.ext.has("development")
    applicationDefaultJvmArgs = listOf("-Dio.ktor.development=$isDevelopment")
}
jib {
    from {
        image = "azul/zulu-openjdk-alpine:17-jre-headless-latest"
        platforms {
            this.platform {
                os = "linux"
                architecture = "amd64"
            }
            this.platform {
                os = "linux"
                architecture = "arm64"
            }
        }
    }
    to {
        image = "reg.phyrone.de/project-kiwi/gateway"
        tags = setOf("latest", project.version.toString())
    }

    container {
        mainClass = "de.phyrone.kiwi.gateway.Main"
        ports = listOf("7080")
    }
}