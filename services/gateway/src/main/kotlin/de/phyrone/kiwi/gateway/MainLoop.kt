package de.phyrone.kiwi.gateway

import com.google.common.flogger.FluentLogger
import kotlinx.coroutines.MainCoroutineDispatcher
import kotlinx.coroutines.Runnable
import java.util.concurrent.*
import kotlin.coroutines.CoroutineContext
import kotlin.system.exitProcess

object MainLoop : MainCoroutineDispatcher(), Executor {
    private val taskQueue = LinkedBlockingQueue<Runnable>()
    private val logger = FluentLogger.forEnclosingClass()
    private var mainThread: Thread? = null

    @Synchronized
    internal fun run_loop() {
        try {
            while (true) {
                val task = taskQueue.take()
                try {
                    task.run()
                } catch (e: InterruptedException) {
                    throw e
                } catch (e: Exception) {
                    logger.atWarning()
                        .withCause(e)
                        .log("Error while executing task")
                }
            }
        } finally {
            mainThread = null
            exitProcess(0)
        }

    }

    override val immediate: MainCoroutineDispatcher = this

    override fun dispatch(context: CoroutineContext, block: Runnable) = execute(block)

    override fun execute(command: Runnable) {
        if (Thread.currentThread() == mainThread) {
            command.run()
            return
        } else {
            taskQueue.add(command)
        }
    }

}