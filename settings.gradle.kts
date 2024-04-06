plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.5.0"
}
rootProject.name = "project-kiwi"

include(
    ":gateway",
    ":audio",
    ":frontend",
    ":shared",
    ":shared:common",
    ":shared:proto",
    ":shared:database",
    ":shared:permission",
    ":shared:metadata",
)
