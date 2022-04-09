import kotlin.math.abs

data class HexVector(
    val s: Int = 0,
    val q: Int = 0,
    val r: Int = 0
) {
    operator fun plus(other: HexVector): HexVector =
        HexVector(
            s = this.s + other.s,
            q = this.q + other.q,
            r = this.r + other.r
        )

    operator fun minus(other: HexVector): HexVector =
        HexVector(
            s = this.s - other.s,
            q = this.q - other.q,
            r = this.r - other.r
        )

    fun distance(other: HexVector = HexVector()): Int {
        val difference = this - other
        return (abs(difference.s) + abs(difference.q) + abs(difference.r)) / 2
    }
}

enum class HexDirection(val id: String, val vector: HexVector) {
    North("n", HexVector(1, 0, -1)),
    NorthEast("ne", HexVector(0, 1, -1)),
    SouthEast("se", HexVector(-1, 1, 0)),
    South("s", HexVector(-1, 0, 1)),
    SouthWest("sw", HexVector(0, -1, 1)),
    NorthWest("nw", HexVector(1, -1,0));
}


fun String.toHexDirection() =
    HexDirection.values().firstOrNull { it.id == this}

fun readDirections(path: String): List<HexVector> =
    readFile(path)
        .firstOrNull()
        ?.split(",")
        ?.mapNotNull { it.toHexDirection()?.vector }
        ?: emptyList()

fun getFinalCoordinate(vectors: List<HexVector>): HexVector =
    vectors.fold(HexVector()) { acc, vector ->
        acc + vector
    }

fun getMaxDistance(vectors: List<HexVector>): Int {
    var maxDistance = 0
    var currentVector = HexVector()
    for (vector in vectors) {
        currentVector += vector
        val distance = currentVector.distance()
        if (distance > maxDistance) {
            maxDistance = distance
        }
    }
    return maxDistance
}

fun solveDayElevenPartOne() {
    val vectors = readDirections("day_eleven.txt")
    val coordinate = getFinalCoordinate(vectors)
    println(coordinate.distance())
}

fun solveDayElevenPartTwo() {
    val vectors = readDirections("day_eleven.txt")
    println(getMaxDistance(vectors))
}

fun main() {
    solveDayElevenPartTwo()
}