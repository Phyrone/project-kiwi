package de.phyrone.kiwi.gateway.socket

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import de.phyrone.kiwi.gateway.packets.WsPacketClientbound
import de.phyrone.kiwi.gateway.packets.WsPacketServerbound
import io.vertx.core.buffer.Buffer
import io.vertx.core.http.ServerWebSocket

class WsGatewaySocket(
    val jsonMapper: ObjectMapper,
    val socket: ServerWebSocket,
    val binaryMapper: ObjectMapper? = null,
) : AbstractPreparedGatewaySocket() {


    init {
        socket.textMessageHandler { text ->
            val packet = jsonMapper.readValue<WsPacketClientbound>(text)
            handleIncommingMessage(packet)
        }
        socket.binaryMessageHandler { buffer ->
            if (binaryMapper == null) {
                close(1008, "no binary support")
                return@binaryMessageHandler
            } else {
                val packet = binaryMapper.readValue<WsPacketClientbound>(buffer.bytes)
                handleIncommingMessage(packet)

            }
        }
        socket.closeHandler {
            super.close()
        }
    }

    override suspend fun send(packet: WsPacketServerbound) {
        if (binaryMapper == null) {
            val message = jsonMapper.writeValueAsString(packet)
            socket.writeTextMessage(message)
        } else {
            val message = binaryMapper.writeValueAsBytes(packet)
            socket.writeBinaryMessage(Buffer.buffer(message))
        }
    }

    override fun close(code: Short, reason: String) {
        super.close()
        socket.close(code, reason)
    }

    override fun close() {
        super.close()
        socket.close()
    }
}