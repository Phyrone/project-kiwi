package de.phyrone.kiwi.common.systems

interface ShutdownHook {
    suspend fun onShutdown()
}