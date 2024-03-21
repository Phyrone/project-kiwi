package de.phyrone.kiwi.gateway.socket

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import de.phyrone.kiwi.gateway.packets.WsPacketClientbound
import de.phyrone.kiwi.gateway.packets.WsPacketServerbound
import io.vertx.ext.web.handler.sockjs.SockJSSocket
import io.vertx.kotlin.coroutines.coAwait


class SockJsGatewaySocket(
    private val objectMapper: ObjectMapper,
    private val sockJs: SockJSSocket,
) : AbstractPreparedGatewaySocket() {

    init {
        sockJs.handler { buffer ->
            val packet = objectMapper.readValue<WsPacketClientbound>(buffer.toString(Charsets.UTF_8))
            handleIncommingMessage(packet)
        }
        sockJs.closeHandler {
            super.close()
        }
    }

    override suspend fun send(packet: WsPacketServerbound) {
        val message = objectMapper.writeValueAsString(packet)
        sockJs.write(message).coAwait()
    }

    override fun close(code: Short, reason: String) {
        super.close()
        sockJs.close(code.toInt(), reason)
    }

    override fun close() {
        super.close()
        sockJs.close()
    }
}