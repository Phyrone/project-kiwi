plugins {
    base
}

tasks {

    val buildBun = create("buildBun", Exec::class) {
        group = "build"
        environment(
            mapOf(
                "BUILD_TARGET" to "bun",
            )
        )
        commandLine("bun", "run", "build")
    }

    build {
        dependsOn(buildBun)

    }

    clean {
        this.delete(
            "node_modules",
            "dist",
            ".svelte-kit"
        )
    }
}