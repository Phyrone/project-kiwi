package de.phyrone.kiwi.gateway

import com.google.common.flogger.FluentLogger
import kotlinx.coroutines.MainCoroutineDispatcher
import kotlinx.coroutines.Runnable
import java.util.concurrent.Executor
import java.util.concurrent.LinkedBlockingQueue
import kotlin.coroutines.CoroutineContext

object MainLoop : MainCoroutineDispatcher(), Executor {
    private val taskQueue = LinkedBlockingQueue<Runnable>()
    private val logger = FluentLogger.forEnclosingClass()

    @Synchronized
    internal fun run_loop() {
        while (true) {
            val task = taskQueue.take()
            try {
                task.run()
            } catch (e: Exception) {
                logger.atWarning()
                    .withCause(e)
                    .log("Error while executing task")
            }
        }
    }

    override val immediate: MainCoroutineDispatcher = this

    override fun dispatch(context: CoroutineContext, block: Runnable) {
        taskQueue.add(block)
    }

    override fun execute(command: java.lang.Runnable) {
        taskQueue.add(command)
    }

}