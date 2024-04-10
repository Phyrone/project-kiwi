plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.5.0"
}
rootProject.name = "project-kiwi"

include(
    ":gateway",
    ":shared",
    ":shared:common",
    ":shared:proto",
    ":shared:database",
    ":shared:permission",
    ":shared:metadata",
    ":media",
)

if(System.getProperty("monorepo.cargo")?.toBooleanStrictOrNull() != false){
    include(":relay")
}
if(System.getProperty("monorepo.node")?.toBooleanStrictOrNull() != false){
    include(":frontend")
}