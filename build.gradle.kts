buildscript {
    dependencies {
        classpath("org.postgresql:postgresql:42.7.3")
        classpath("org.flywaydb:flyway-core:10.10.0")
        classpath("org.flywaydb:flyway-database-postgresql:10.4.1")
    }
}
plugins {
    idea
    base
    id("com.github.ben-manes.versions") version "0.51.0"

    // Used in Subprojects
    kotlin("jvm") version "1.9.23" apply false
    kotlin("kapt") version "1.9.23" apply false
    kotlin("multiplatform") version "1.9.23" apply false
    id("org.flywaydb.flyway") version "10.10.0" apply false
    id("com.google.cloud.tools.jib") version "3.4.2" apply false
    id("com.google.devtools.ksp") version "1.9.23-1.0.20" apply false
    id("com.github.node-gradle.node") version "7.0.2" apply false

}

group = "de.phyrone"
version = "0.0.1-INDEV"

tasks {

    wrapper {
        gradleVersion = "8.7"
        distributionType = Wrapper.DistributionType.ALL
    }
    clean{
        this.setDelete("target")
        this.setDelete("build")
    }

}

/*
repositories {
    mavenCentral()
}

dependencies {
    testImplementation("org.jetbrains.kotlin:kotlin-test")
}

tasks.test {
    useJUnitPlatform()
}
kotlin {
    jvmToolchain(17)
}

 */