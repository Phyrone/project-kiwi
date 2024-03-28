package de.phyrone.kiwi.gateway

import de.phyrone.kiwi.common.CommonsModule
import de.phyrone.kiwi.database.DatabaseModule
import org.koin.core.annotation.ComponentScan
import org.koin.core.annotation.Module

@Module(includes = [DatabaseModule::class,CommonsModule::class])
@ComponentScan
object ModuleBundle