plugins {
    kotlin("jvm") version "1.9.22" apply false
    id("com.github.ben-manes.versions") version "0.51.0"
}

group = "de.phyrone"
version = "1.0-SNAPSHOT"


tasks{

    wrapper{
        gradleVersion = "8.6"
        distributionType = Wrapper.DistributionType.ALL
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