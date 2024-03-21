package de.phyrone.kiwi.gateway

import picocli.CommandLine.Command
import picocli.CommandLine.Option
import java.net.InetSocketAddress

@Command()
class StartupParams : Runnable {

    @Option(
        names = ["-b", "--bind"],
        description = ["The address to bind to"],
        required = true,
        defaultValue = "0.0.0.0:7080",
    )
    lateinit var binds: List<String>
    override fun run() = Main.runApplication(this)

}