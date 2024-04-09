import com.github.gradle.node.yarn.task.YarnTask

plugins {
    base
    id("com.github.node-gradle.node")
}

tasks {

    val buildBun = create<YarnTask>("build-bun") {
        dependsOn(yarn)
        group = "build"
        this.environment = mapOf(
            "NODE_ENV" to "production",
            "BUILD_TARGET" to "bun"
        )
        this.args = listOf("run", "build")
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