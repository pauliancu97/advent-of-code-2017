data class AlgorithmState(
    val list: List<Int> = emptyList(),
    val position: Int = 0,
    val skipSize: Int = 0
)

private val SUFFIX = listOf(17, 31, 73, 47, 23)

fun getSelectedValues(list: List<Int>, start: Int, length: Int): List<Int> =
    if (start + length <= list.size) {
        list.subList(start, start + length)
    } else {
        val firstPart = list.subList(start, list.size)
        val remainingLength = length - (list.size - start)
        val secondPart = list.subList(0, remainingLength)
        firstPart + secondPart
    }


fun getReversed(list: List<Int>, start: Int, length: Int): List<Int> {
    val reversedSelectedValues = getSelectedValues(list, start, length).reversed()
    val result = list.toMutableList()
    for (offset in 0 until length) {
        val index = (start + offset % list.size) % list.size
        result[index] = reversedSelectedValues[offset]
    }
    return result
}


fun getUpdatedAlgorithmState(state: AlgorithmState, length: Int): AlgorithmState =
    AlgorithmState(
        list = getReversed(
            list = state.list,
            start = state.position,
            length = length
        ),
        position = (state.position + (state.skipSize % state.list.size) + (length % state.list.size)) % state.list.size,
        skipSize = state.skipSize + 1
    )


fun getUpdatedAlgorithmState(state: AlgorithmState, lengths: List<Int>): AlgorithmState =
    lengths
        .fold(state) { currentState, length ->
            getUpdatedAlgorithmState(currentState, length)
        }

fun readLengths(path: String): List<Int> =
    readFile(path)
        .firstOrNull()
        ?.split(",")
        ?.mapNotNull { it.toIntOrNull() }
        ?: emptyList()

fun getReadInput(path: String): List<Int> =
    (readFile(path)
        .firstOrNull()
        ?.map { it.code }
        ?: emptyList()) + SUFFIX

fun getSparseHash(input: List<Int>): List<Int> {
    var currentState = AlgorithmState(List(256) { it })
    repeat(64) {
        currentState = getUpdatedAlgorithmState(currentState, input)
    }
    return currentState.list
}

fun getDenseHash(sparseHash: List<Int>): List<Int> =
    sparseHash
        .chunked(16)
        .map { chunk ->
            chunk.reduce { acc, i -> acc xor i }
        }

fun UByte.toHex(): String =
    "%x".format(this.toByte())

fun Int.toHex(): String {
    val firstByte = (this and 0x0f).toUByte()
    val secondByte = ((this shr 4) and 0x0f).toUByte()
    return secondByte.toHex() + firstByte.toHex()
}

fun List<Int>.toHex(): String =
    this.joinToString(transform = { it.toHex() }, separator = "")

fun solveDayTenPartOne() {
    val lengths = readLengths("day_ten.txt")
    val finalState = getUpdatedAlgorithmState(
        state = AlgorithmState(List(256) { it }),
        lengths = lengths
    )
    val result = finalState.list[0] * finalState.list[1]
    println(result)
}

fun solveDayTenPartTwo() {
    val input = getReadInput("day_ten.txt")
    val sparseHash = getSparseHash(input)
    val denseHash = getDenseHash(sparseHash)
    val result = denseHash.toHex()
    println(result)
}

fun main() {
    solveDayTenPartTwo()
}