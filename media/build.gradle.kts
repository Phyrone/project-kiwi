plugins {
    kotlin("jvm")
}

group = parent?.group ?: error("parent not specified")
version = parent?.version ?: error("parent not specified")