package de.phyrone.kiwi.common

import de.mkammerer.argon2.Argon2Factory
import de.phyrone.kiwi.common.config.ConfigModule
import org.koin.core.annotation.ComponentScan
import org.koin.core.annotation.Module

@Module(includes = [ConfigModule::class])
@ComponentScan
class CommonsModule{


    fun argon2() = Argon2Factory.create(Argon2Factory.Argon2Types.ARGON2id)
}