plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.5.0"
}
rootProject.name = "project-kiwi-c2"

include(
    ":gateway",
    ":shared:common",
    ":shared:proto",
    ":shared:database"
)
