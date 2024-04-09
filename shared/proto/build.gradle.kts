plugins {
    kotlin("jvm")
    id("com.google.protobuf") version "0.9.4"
}

repositories {
    mavenCentral()
}

dependencies {
    runtimeOnly("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1-Beta")

    /* Protobuf */
    api("com.google.protobuf:protobuf-java:4.26.0")
    api("com.google.protobuf:protobuf-java-util:4.26.0")
    api("com.google.protobuf:protobuf-kotlin:4.26.1")

    /* gRPC */
    api("io.grpc:grpc-protobuf:1.62.2")
    api("io.grpc:grpc-netty:1.62.2")
    api("io.grpc:grpc-all:1.62.2")

    api("io.grpc:grpc-stub:1.62.2")
    api("io.grpc:grpc-kotlin-stub:1.4.1")


}

sourceSets {
    main {
        proto {
            srcDirs("../../proto")
        }
    }
}
protobuf {

    protoc {
        artifact = "com.google.protobuf:protoc:4.26.0"
    }

    plugins {
        create("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:1.62.2"
        }
        create("grpckt") {
            artifact = "io.grpc:protoc-gen-grpc-kotlin:1.4.1:jdk8@jar"
        }
    }
    generateProtoTasks {
        all().forEach {
            it.plugins {
                create("grpc")
                create("grpckt")
            }
            it.builtins {
                create("kotlin")
            }
        }
    }
}