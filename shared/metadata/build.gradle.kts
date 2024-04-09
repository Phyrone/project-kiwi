import com.google.devtools.ksp.gradle.KspTaskMetadata
import de.phyrone.kiwi.build.GitData
import de.phyrone.kiwi.build.MetadataBuilder
import de.phyrone.kiwi.build.SemanticVersion
import org.jetbrains.kotlin.gradle.dsl.KotlinJsCompile
import org.jetbrains.kotlin.gradle.targets.js.dsl.ExperimentalWasmDsl
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import org.jetbrains.kotlin.gradle.tasks.KotlinCompileCommon
import java.time.Instant

plugins {
    kotlin("multiplatform")
}

group = parent!!.group
version = parent!!.version


val generatedMetadataDir = layout.buildDirectory.dir("generated/metadata").get()
repositories {
    mavenCentral()
}

@OptIn(ExperimentalWasmDsl::class)
kotlin {
    jvm("jvm") {
        //withJava()
    }
    js("js") {
        browser()
        nodejs()
        binaries.library()
        generateTypeScriptDefinitions()
    }
    /*
    wasmJs("wasm") {
        browser()
        nodejs()
        binaries.library()
        generateTypeScriptDefinitions()
    }
     */

    sourceSets {
        val commonMain by getting {
            this.kotlin.srcDir(generatedMetadataDir)
            dependencies {
                implementation(kotlin("stdlib"))
            }
        }
        val jvmMain by getting {
            dependencies {
                implementation(kotlin("stdlib-jdk8"))
            }
        }
        val jsMain by getting {
            dependencies {
                implementation(kotlin("stdlib-js"))
            }
        }
        /*
        val wasmMain by getting {
            dependencies {
                implementation(kotlin("stdlib-wasm-js"))
            }
        }
         */
    }
}

tasks {
    val buildMetadataTask = create("build-metadata") {
        val semantic = SemanticVersion.parse(project.version as String)
        val gitData = GitData(rootDir)
        val data = mapOf(
            "version" to project.version,
            "version-major" to semantic.major,
            "version-minor" to semantic.minor,
            "version-patch" to semantic.patch,
            "version-prerelease" to (semantic.preRelease ?: ""),
            "build-timestamp" to Instant.now().toString(),
            "git-branch" to gitData.branch(),
            "git-commit" to gitData.lastCommit().id.name()
        )
        inputs.property("data", data)
        outputs.dir(generatedMetadataDir)
        doLast {
            MetadataBuilder.buildMetadata(data, generatedMetadataDir.asFile)
        }
    }


    withType<KotlinCompileCommon>() {
        dependsOn(buildMetadataTask)
    }
    withType<KspTaskMetadata>() {
        dependsOn(buildMetadataTask)
    }
    withType<KotlinJsCompile>() {
        dependsOn(buildMetadataTask)
    }
    withType<KotlinCompile>() {
        dependsOn(buildMetadataTask)
    }

}