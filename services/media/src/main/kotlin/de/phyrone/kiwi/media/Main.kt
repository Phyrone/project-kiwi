package de.phyrone.kiwi.media

import picocli.CommandLine

object Main{

    @JvmStatic
    fun main(args: Array<String>) {
        val commandLine = CommandLine(StartupParams::class.java)

    }
}