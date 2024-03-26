package de.phyrone.kiwi.common.systems


interface StartupRunner {
    suspend fun onStartup()
}