plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.5.0"
}
rootProject.name = "project-kiwi"

include(
    ":shared",
    ":shared:common",
    ":shared:proto",
    ":shared:database",
    ":shared:permission",
    ":shared:metadata",

    // Services
    ":services:gateway",
    ":services:media"
)

if(System.getProperty("monorepo.cargo")?.toBooleanStrictOrNull() != false){
    include(":services:relay")
}
if(System.getProperty("monorepo.node")?.toBooleanStrictOrNull() != false){
    include(":services:frontend")
}