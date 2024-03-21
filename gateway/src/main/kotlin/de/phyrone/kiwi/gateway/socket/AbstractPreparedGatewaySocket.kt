package de.phyrone.kiwi.gateway.socket

import de.phyrone.kiwi.gateway.packets.WsPacketClientbound
import kotlinx.coroutines.channels.BufferOverflow
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.channels.ReceiveChannel
import kotlinx.coroutines.runBlocking

abstract class AbstractPreparedGatewaySocket : GatewaySocket {
    private val ingressChannel =
        Channel<WsPacketClientbound>(Channel.BUFFERED, onBufferOverflow = BufferOverflow.SUSPEND)
    override val receive: ReceiveChannel<WsPacketClientbound> = ingressChannel

    protected fun handleIncommingMessage(packet: WsPacketClientbound) {
        runBlocking {
            ingressChannel.send(packet)
        }
    }

    override fun close(code: Short, reason: String) {
        ingressChannel.close()
    }

    override fun close() {
        ingressChannel.close()
    }

}