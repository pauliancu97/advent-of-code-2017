import java.io.File

fun readFile(path: String): List<String> =
    File(path).readLines()

fun getDigitsCircular(line: String): List<Int> {
    val digits = line.map { it.toInt() - '0'.toInt() }
    return digits + digits.first()
}

fun getDigits(line: String) = line.map { it.toInt() - '0'.toInt() }

fun getSumMatchingDigits(digits: List<Int>): Int {
    val firstDigits = digits.dropLast(1)
    val lastDigits = digits.drop(1)
    return firstDigits.zip(lastDigits)
        .mapNotNull { (first, last) ->
            if (first == last) first else null
        }
        .sum()
}

fun getSumMatchingDigitsHalfAway(digits: List<Int>): Int {
    val step = digits.size / 2
    var sum = 0
    for ((firstIndex, firstDigit) in digits.withIndex()) {
        val secondIndex = (firstIndex + step) % digits.size
        val secondDigit = digits[secondIndex]
        if (firstDigit == secondDigit) {
            sum += firstDigit
        }
    }
    return sum
}

fun solveDayOnePartOne() {
    val lines = readFile("day_one.txt")
    val digits = getDigitsCircular(lines[0])
    val result = getSumMatchingDigits(digits)
    println(result)
}

fun solveDayOnePartTwo() {
    val lines = readFile("day_one.txt")
    val digits = getDigits(lines[0])
    val result = getSumMatchingDigitsHalfAway(digits)
    println(result)
}

fun main() {
    solveDayOnePartTwo()
}