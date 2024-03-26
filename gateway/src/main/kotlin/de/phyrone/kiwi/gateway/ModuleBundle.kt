package de.phyrone.kiwi.gateway

import de.phyrone.kiwi.common.CommonModule
import de.phyrone.kiwi.database.DatabaseModule
import org.koin.core.annotation.Module

@Module(
    includes = [
        CommonModule::class,
        DatabaseModule::class
    ]
)
object ModuleBundle