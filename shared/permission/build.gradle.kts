plugins{
    kotlin("jvm")
    id("com.google.devtools.ksp")
}

repositories {
    mavenCentral()
}

dependencies{
    implementation(project(":shared:database"))

    implementation("io.insert-koin:koin-core-coroutines:3.6.0-wasm-alpha2")
    implementation("io.insert-koin:koin-annotations:1.3.1")
    ksp("io.insert-koin:koin-ksp-compiler:1.3.1")

    implementation("org.casbin:jcasbin:1.54.0")

}