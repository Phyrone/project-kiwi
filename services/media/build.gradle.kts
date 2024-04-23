plugins {
    application
    kotlin("jvm")
    kotlin("kapt")
}

repositories {
    mavenCentral()

}
dependencies {
    implementation(project(":shared:common"))
    implementation(project(":shared:database"))
    implementation(project(":shared:metadata"))
    implementation(kotlin("stdlib-jdk8"))
    implementation(kotlin("reflect"))


    implementation("info.picocli:picocli:4.7.5")
    kapt("info.picocli:picocli:4.7.5")

}
application{

}