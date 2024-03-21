plugins {
    kotlin("jvm")
    kotlin("kapt")
    application

}

group = parent?.group ?: error("parent not specified")
version = parent?.version ?: error("parent not specified")

repositories {
    mavenCentral()
}


dependencies {

    /* Kotlin */
    implementation(kotlin("stdlib-jdk8"))
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1-Beta")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-jdk8:1.8.1-Beta")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-reactor:1.8.1-Beta")

    /* Vert.x */
    implementation(platform("io.vertx:vertx-stack-depchain:4.5.5"))
    implementation("io.vertx:vertx-auth-jwt")
    implementation("io.vertx:vertx-web")
    implementation("io.vertx:vertx-grpc-server")
    //implementation("io.vertx:vertx-service-discovery")
    implementation("io.vertx:vertx-json-schema")
    implementation("io.vertx:vertx-stomp")
    implementation("io.vertx:vertx-sockjs-service-proxy")
    implementation("io.vertx:vertx-grpc-client")
    //implementation("io.vertx:vertx-config")
    implementation("io.vertx:vertx-jdbc-client")
    implementation("io.vertx:vertx-pg-client")
    implementation("io.vertx:vertx-lang-kotlin-coroutines")
    implementation("io.vertx:vertx-lang-kotlin")


    /* Dependency Injection, EventBus, Modules etc. */
    implementation("io.insert-koin:koin-core:3.6.0-wasm-alpha2")
    implementation("io.insert-koin:koin-core-coroutines:3.6.0-wasm-alpha2")
    implementation("org.atteo.classindex:classindex:3.13")
    kapt("org.atteo.classindex:classindex:3.13")

    /* Logging */
    implementation("info.picocli:picocli:4.7.5")
    implementation("org.slf4j:slf4j-api:2.1.0-alpha1")
    implementation("ch.qos.logback:logback-classic:1.5.3")
    implementation("com.google.flogger:flogger:0.8")
    runtimeOnly("com.google.flogger:flogger-slf4j-backend:0.8")

    /* Database */
    implementation("com.rethinkdb:rethinkdb-driver:2.4.4")
    implementation("org.postgresql:r2dbc-postgresql:1.0.4.RELEASE")


    /* Other */
    implementation("com.auth0:java-jwt:4.4.0")

    /* Json, MessagePack, etc. */
    implementation("com.fasterxml.jackson.core:jackson-core:2.17.0")
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.17.0")
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.17.0")
    implementation("org.msgpack:jackson-dataformat-msgpack:0.9.8")

}

kotlin {
    jvmToolchain(17)
}