package de.phyrone.kiwi.common.crypto

import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import kotlin.time.measureTimedValue


class Argon2(
    val memory: Int = 1024 * 16,
    val iterations: Int = 32,
    val parallelism: Int = Runtime.getRuntime().availableProcessors().coerceAtLeast(1) * 2,
) {

    private val encoder = Argon2PasswordEncoder(
        64, 128,
        parallelism,
        memory,
        iterations
    )

    fun create(password: String): String = encoder.encode(password)
    fun verify(password: String, hash: String): Boolean = encoder.matches(password, hash)


    companion object{
        @JvmStatic
        fun main(args: Array<String>) {
            val argon2 = Argon2()
            repeat(10) {
                val password = "password"
                val (hash, time) = measureTimedValue { argon2.create(password) }
                println("Hash: $hash in $time")
                println("Verify: ${argon2.verify(password, hash)}")
            }

        }
    }


}