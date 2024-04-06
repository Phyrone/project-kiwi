package de.phyrone.kiwi.build

data class SemanticVersion(
    val major: Int,
    val minor: Int,
    val patch: Int,
    val preRelease: String? = null,
) : Comparable<SemanticVersion> {
    companion object {
        private val SEM_REGEX = "^(\\d+)\\.(\\d+)\\.(\\d+)(-(\\w+))?$".toRegex()
        fun parse(version: String): SemanticVersion {
            val match = SEM_REGEX.matchEntire(version) ?: error("not a semantic version: $version")
            val (majorStr, minorStr, patchStr, _, preRelease) = match.destructured
            return SemanticVersion(majorStr.toInt(), minorStr.toInt(), patchStr.toInt(), preRelease)
        }
    }

    override fun compareTo(other: SemanticVersion): Int = when {
        this.major != other.major -> this.major.compareTo(other.major)
        this.minor != other.minor -> this.minor.compareTo(other.minor)
        this.patch != other.patch -> this.patch.compareTo(other.patch)
        else -> 0
    }

}
