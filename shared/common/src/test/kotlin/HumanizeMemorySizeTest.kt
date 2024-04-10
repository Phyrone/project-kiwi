import de.phyrone.kiwi.common.humanizeMemorySize
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import java.util.*

//Copilot generated test cases
class HumanizeMemorySizeTest {

    @Test
    fun `should return 0 B when size is 0`() {
        val result = humanizeMemorySize(0, locale = Locale.US)
        assertEquals("0 B", result)
    }

    @Test
    fun `should return humanized size in SI units when si is true`() {
        val result = humanizeMemorySize(1500, locale = Locale.US)
        assertEquals("1.50 KB", result)
    }

    @Test
    fun `should return humanized size in IEC units when si is false`() {
        val result = humanizeMemorySize(1500, si = false, locale = Locale.US)
        assertEquals("1.46 KiB", result)
    }

    @Test
    fun `should return humanized size with specified decimals`() {
        val result = humanizeMemorySize(1500, decimals = 3, locale = Locale.US)
        assertEquals("1.500 KB", result)
    }

    @Test
    fun `should return humanized size of negative value when unsigned is false`() {
        val result = humanizeMemorySize(-1500, locale = Locale.US)
        assertEquals("-1.50 KB", result)
    }

}