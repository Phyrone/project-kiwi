plugins {
    base
}

tasks {
    val cargoBuild = create("cargoBuild", Exec::class) {
        commandLine("cargo", "build")
    }
    build {
        dependsOn(cargoBuild)
    }

}