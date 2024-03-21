package de.phyrone.kiwi.gateway

import de.phyrone.kiwi.gateway.packets.WsPacketClientbound
import de.phyrone.kiwi.gateway.socket.GatewaySocket
import kotlinx.coroutines.coroutineScope
import org.koin.core.Koin
import org.koin.core.component.KoinComponent

class GatewaySession(
    private val koin: Koin,
    private val socket: GatewaySocket,
) : KoinComponent {

    override fun getKoin(): Koin {
        return koin
    }

    suspend fun run() {

        coroutineScope {

            for (packet in socket.receive) {
                handleIncomingPacket(packet)
            }
        }
    }

    private suspend fun handleIncomingPacket(packet: WsPacketClientbound) {
        TODO()
    }

}