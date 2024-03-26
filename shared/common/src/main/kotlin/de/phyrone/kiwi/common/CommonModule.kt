package de.phyrone.kiwi.common

import de.phyrone.kiwi.common.config.ConfigModule
import org.koin.core.annotation.Module

@Module(
    includes = [
        ConfigModule::class
    ]
)
class CommonModule