package de.phyrone.kiwi.gateway

import io.vertx.core.http.ServerWebSocket
import io.vertx.core.http.impl.HttpUtils
import io.vertx.core.net.SocketAddress
import io.vertx.ext.web.RoutingContext


fun handleWebsocket(
    ctx: RoutingContext,
    onUpgrade: (ServerWebSocket) -> Unit,
    onNoUpgrade: (ctx: RoutingContext) -> Unit = {
        ctx.response().setStatusCode(400)
        ctx.response().end("""only websocket requests are allowed here.""")
    }
) {
    val request = ctx.request()

    if (HttpUtils.canUpgradeToWebSocket(request)) {
        request.toWebSocket()
            .onSuccess(onUpgrade)
            .onFailure(ctx::fail)
        return
    } else {
        onNoUpgrade(ctx)
    }
}

private val socketAddressRegex = "([\\w.\\-]+)?:(\\d+)".toRegex()

fun parseSocketAddress(address: String): SocketAddress? {
    val match = socketAddressRegex.matchEntire(address) ?: return null
    val (host, port) = match.destructured
    return SocketAddress.inetSocketAddress(port.toInt(), host)
}

