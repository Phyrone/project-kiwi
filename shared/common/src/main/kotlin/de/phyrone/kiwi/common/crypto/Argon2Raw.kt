package de.phyrone.kiwi.common.crypto

import org.bouncycastle.crypto.generators.Argon2BytesGenerator
import org.bouncycastle.crypto.params.Argon2Parameters
import java.security.SecureRandom

//called it argon2 "raw" because we do not use the text encoding normally used by argon2 but instead use the raw bytes
object Argon2Raw {

    private val random = SecureRandom.getInstanceStrong()


    data class HashedPassword(
        val hash: ByteArray,
        val salt: ByteArray,
        val iterations: Int,
        val memory: Int,
        val parallelism: Int
    ) {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (javaClass != other?.javaClass) return false

            other as HashedPassword

            if (!hash.contentEquals(other.hash)) return false
            if (!salt.contentEquals(other.salt)) return false
            if (memory != other.memory) return false
            if (iterations != other.iterations) return false
            if (parallelism != other.parallelism) return false

            return true
        }

        override fun hashCode(): Int {
            var result = hash.contentHashCode()
            result = 31 * result + salt.contentHashCode()
            result = 31 * result + memory
            result = 31 * result + iterations
            result = 31 * result + parallelism
            return result
        }
    }


    fun create(
        password: String,
        length: Int = 64,
        saltLength: Int = 32,
        memory: Int = 1024 * 4,
        iterations: Int = 3,
        parallelism: Int = Runtime.getRuntime().availableProcessors().coerceAtLeast(1) * 2
    ): HashedPassword {
        val salt = ByteArray(saltLength)
        random.nextBytes(salt)

        val hash = ByteArray(length)
        val generator = Argon2BytesGenerator()
        generator.init(
            Argon2Parameters.Builder(Argon2Parameters.ARGON2_id)
                .withVersion(Argon2Parameters.ARGON2_VERSION_13)
                .withMemoryAsKB(memory)
                .withParallelism(parallelism)
                .withIterations(iterations)
                .withSalt(salt)
                .build()
        )
        generator.generateBytes(password.toByteArray(Charsets.UTF_8), hash)

        return HashedPassword(hash, salt, iterations, memory, parallelism)
    }

    fun verify(password: String, argon2Raw: HashedPassword): Boolean {
        val hash = ByteArray(argon2Raw.hash.size)
        val generator = Argon2BytesGenerator()
        generator.init(
            Argon2Parameters.Builder(Argon2Parameters.ARGON2_id)
                .withVersion(Argon2Parameters.ARGON2_VERSION_13)
                .withMemoryAsKB(argon2Raw.memory)
                .withParallelism(argon2Raw.parallelism)
                .withIterations(argon2Raw.iterations)
                .withSalt(argon2Raw.salt)
                .build()
        )
        generator.generateBytes(password.toByteArray(Charsets.UTF_8), hash)
        return hash.contentEquals(argon2Raw.hash)
    }
}