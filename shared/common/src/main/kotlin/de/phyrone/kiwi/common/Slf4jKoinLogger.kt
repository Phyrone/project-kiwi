package de.phyrone.kiwi.common

import org.koin.core.Koin
import org.koin.core.logger.MESSAGE
import org.koin.core.logger.Level as KoinLevel
import org.koin.core.logger.Logger as KoinLogger


private val logger = loggerFor<Koin>()
private fun getLevel(): KoinLevel {
    return when {
        logger.atFiner().isEnabled -> KoinLevel.DEBUG
        logger.atFine().isEnabled -> KoinLevel.INFO
        logger.atWarning().isEnabled -> KoinLevel.WARNING
        logger.atSevere().isEnabled -> KoinLevel.ERROR
        else -> KoinLevel.NONE
    }
}

object Slf4jKoinLogger : KoinLogger(getLevel()) {

    override fun display(level: KoinLevel, msg: MESSAGE) {
        when (level) {
            KoinLevel.DEBUG -> logger.atFiner().log(msg)
            KoinLevel.INFO -> logger.atFine().log(msg)
            KoinLevel.WARNING -> logger.atWarning().log(msg)
            KoinLevel.ERROR -> logger.atSevere().log(msg)
            KoinLevel.NONE -> {}
        }
    }

}