import org.flywaydb.gradle.task.FlywayCleanTask
import org.flywaydb.gradle.task.FlywayMigrateTask

buildscript {
    dependencies {
        classpath("org.postgresql:postgresql:42.7.3")
        classpath("org.flywaydb:flyway-core:10.10.0")
        classpath("org.flywaydb:flyway-database-postgresql:10.4.1")
    }
}
plugins {
    kotlin("jvm")
    id("org.flywaydb.flyway")
    id("com.google.devtools.ksp")
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(":shared:common"))


    /* Reactive Driver */
    api("org.postgresql:r2dbc-postgresql:1.0.4.RELEASE")

    /* JDBC Based (Blocking) */
    implementation("org.flywaydb:flyway-core:10.10.0")
    implementation("org.flywaydb:flyway-database-postgresql:10.4.1")

    /* JDBC Driver */
    runtimeOnly("org.postgresql:postgresql:42.7.3")

    /* HikariCP */
    implementation("com.zaxxer:HikariCP:5.1.0")

    /* Dependency Injection, EventBus, Modules etc. */
    implementation("io.insert-koin:koin-core-coroutines:3.6.0-wasm-alpha2")
    implementation("io.insert-koin:koin-annotations:1.3.1")
    ksp("io.insert-koin:koin-ksp-compiler:1.3.1")

}

tasks {
    val flywayMigrate by withType(FlywayMigrateTask::class)
    val flywayClean by withType(FlywayCleanTask::class)
    task("flywayFresh") {
        group = "Flyway"
        dependsOn(flywayClean)
        finalizedBy(flywayMigrate)
    }
}

flyway {
    locations = arrayOf("filesystem:src/main/resources/files/db/migration")

    url = "jdbc:postgresql://localhost/app2"
    user = "app2"
    password = "123456"
    schemas = arrayOf("public")
    driver = "org.postgresql.Driver"
    cleanDisabled = false

}