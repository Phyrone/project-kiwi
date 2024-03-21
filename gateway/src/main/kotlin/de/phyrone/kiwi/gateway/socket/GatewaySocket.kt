package de.phyrone.kiwi.gateway.socket

import de.phyrone.kiwi.gateway.packets.WsPacketClientbound
import de.phyrone.kiwi.gateway.packets.WsPacketServerbound
import kotlinx.coroutines.channels.ReceiveChannel
import java.io.Closeable

interface GatewaySocket : Closeable {
    val receive: ReceiveChannel<WsPacketClientbound>
    suspend fun send(packet: WsPacketServerbound)

    fun close(code: Short, reason: String)
}