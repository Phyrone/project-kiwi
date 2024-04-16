plugins {
    kotlin("jvm")
    kotlin("kapt")
    id("com.google.devtools.ksp")
}

repositories {
    mavenCentral()
}

dependencies {

    implementation(kotlin("stdlib"))
    implementation(kotlin("stdlib-jdk8"))
    implementation(kotlin("reflect"))

    testImplementation(kotlin("test"))

    api("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1-Beta")
    api("org.jetbrains.kotlinx:kotlinx-coroutines-jdk8:1.8.1-Beta")
    api("org.jetbrains.kotlinx:kotlinx-coroutines-reactor:1.8.1-Beta")
    api("org.jetbrains.kotlinx:kotlinx-coroutines-reactive:1.8.1-Beta")


    /* Dependency Injection, EventBus, Modules etc. */
    implementation("io.insert-koin:koin-core-coroutines:3.6.0-wasm-alpha2")
    implementation("io.insert-koin:koin-annotations:1.3.1")
    ksp("io.insert-koin:koin-ksp-compiler:1.3.1")

    api("org.atteo.classindex:classindex:3.13")
    kapt("org.atteo.classindex:classindex:3.13")

    /* Logging */
    api("org.slf4j:slf4j-api:2.1.0-alpha1")
    api("com.google.flogger:flogger:0.8")
    implementation("ch.qos.logback:logback-classic:1.5.4")
    runtimeOnly("com.google.flogger:flogger-slf4j-backend:0.8")
    runtimeOnly("commons-logging:commons-logging:1.3.1")

    /* Config */
    api("io.github.config4k:config4k:0.6.0")

    /* Crypto */
    api("de.mkammerer:argon2-jvm:2.7")
    //implementation("org.bouncycastle:bcprov-jdk18on:1.78")
    //implementation("org.springframework.security:spring-security-crypto:6.2.3")


}

tasks{
    test{
        useJUnitPlatform()
    }
}

kotlin{
    jvmToolchain(17)
}

ksp{
    this.arg("KOIN_DEFAULT_MODULE","false")
}