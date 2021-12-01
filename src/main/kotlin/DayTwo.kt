fun getSpreadSheet(lines: List<String>): List<List<Int>> {
    val numberRegex = """\d+""".toRegex()
    return lines
        .map { line ->
            numberRegex.findAll(line)
                .map { matchResult ->
                    matchResult.value.toInt()
                }
                .toList()
        }
}

fun getSpreadSheetCheckSum(spreadSheet: List<List<Int>>): Int =
    spreadSheet
        .map { row ->
            (row.maxOrNull() ?: 0) - (row.minOrNull() ?: 0)
        }
        .sum()

fun getSpreadSheetSecondCheckSum(spreadSheet: List<List<Int>>): Int =
    spreadSheet
        .map { row ->
            var result = 0
            for ((firstIndex, first) in row.withIndex()) {
                for ((secondIndex, second) in row.withIndex()) {
                    if (firstIndex != secondIndex && second % first == 0 && result == 0) {
                        result = second / first
                    }
                }
            }
            result
        }
        .sum()

fun solveDayTwoPartOne() {
    val lines = readFile("day_two.txt")
    val spreadSheet = getSpreadSheet(lines)
    val result = getSpreadSheetCheckSum(spreadSheet)
    println(result)
}

fun solveDayTwoPartTwo() {
    val lines = readFile("day_two.txt")
    val spreadSheet = getSpreadSheet(lines)
    val result = getSpreadSheetSecondCheckSum(spreadSheet)
    println(result)
}

fun main() {
    solveDayTwoPartTwo()
}