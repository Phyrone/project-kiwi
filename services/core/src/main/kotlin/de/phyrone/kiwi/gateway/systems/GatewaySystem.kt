package de.phyrone.kiwi.gateway.services

import org.koin.core.Koin
import org.koin.core.component.KoinComponent

abstract class GatewaySystem(private val koin: Koin) : KoinComponent {
    override fun getKoin(): Koin = koin

}