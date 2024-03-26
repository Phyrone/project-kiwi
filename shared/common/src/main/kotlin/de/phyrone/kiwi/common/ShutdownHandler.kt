package de.phyrone.kiwi.common

import de.phyrone.kiwi.common.systems.ShutdownHook
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.runBlocking
import org.koin.core.Koin
import java.util.concurrent.*
import kotlin.concurrent.thread
import kotlin.coroutines.CoroutineContext
import kotlin.time.measureTime

class ShutdownHandler(
    private val koin: Koin,
    threadCount: Int = deafaultThreadCount(),
) {

    private val task = ShutdownTask(threadCount)

    fun register() {
        Runtime.getRuntime().addShutdownHook(thread(start = false, name = "Shutdown-Main") {
            task.runShutdown()
        })
    }

    private inner class ShutdownTask(
        val threadCount: Int
    ) : CoroutineDispatcher() {
        val threads = Array(threadCount) { threadID -> ShutdownThread(this, threadID + 1).register() }
        val queue = LinkedBlockingQueue<Runnable>()
        override fun dispatch(context: CoroutineContext, block: Runnable) {
            queue.add(block)
        }

        fun runShutdown() {
            val shutdownTime = measureTime {
                try {
                    runBlocking(this) {
                        koin.getAll<ShutdownHook>()
                            .executeOrdered(ExecutionOrder.BACKWARD) { hook -> hook.onShutdown() }
                    }
                } finally {
                    threads.forEach { it.interrupt() }
                }
            }
            logger.atInfo().log("Shutdown done (%s)", shutdownTime)
            shutdownLogger()
        }

    }

    private class ShutdownThread(
        private val task: ShutdownTask,
        id: Int
    ) : Thread("Shutdown-Worker-$id") {
        override fun run() {
            while (true) {
                try {
                    task.queue.take().run()
                } catch (e: InterruptedException) {
                    break
                } catch (e: Throwable) {
                    logger.atSevere().withCause(e).log("Error in shutdown task")
                }
            }
        }

        fun register(): ShutdownThread {
            Runtime.getRuntime().addShutdownHook(this)
            return this
        }
    }

    companion object {
        private val logger = logger()
        fun deafaultThreadCount() = Runtime.getRuntime().availableProcessors().coerceIn(1..5) * 2
    }
}