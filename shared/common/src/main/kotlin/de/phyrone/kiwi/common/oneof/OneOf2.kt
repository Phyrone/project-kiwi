package de.phyrone.kiwi.common.oneof

interface OneOf2<T1, T2> {
    data class Variant1<T1, T2>(val value: T1) : OneOf2<T1, T2>
    data class Variant2<T1, T2>(val value: T2) : OneOf2<T1, T2>

    fun isVariant1(): Boolean = this is Variant1

    fun isVariant2(): Boolean = this is Variant2

    fun variant1OrNull(): T1? = (this as? Variant1)?.value

    fun variant2OrNull(): T2? = (this as? Variant2)?.value

    fun variant1(): T1 = variant1OrNull() ?: error("this is not Variant1")

    fun variant2(): T2 = variant2OrNull() ?: error("this is not Variant2")


}