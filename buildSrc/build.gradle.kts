plugins {
    idea
    `kotlin-dsl`
}
repositories {
    mavenCentral()
}
dependencies {
    implementation("com.squareup:kotlinpoet:1.16.0")
    implementation("org.eclipse.jgit:org.eclipse.jgit:6.9.0.202403050737-r")
}