sealed class Pattern {
    open val depth: Int = 0

    data class Group(override val depth: Int = 0, val patterns: List<Pattern> = emptyList()) : Pattern()
    data class Garbage(override val depth: Int = 0, val content: String) : Pattern()
}

fun parseGarbage(string: String, parentDepth: Int): Pair<Pattern.Garbage, String> {
    var index = 1
    var content = ""
    while (index < string.length && string[index] != '>') {
        if (string[index] == '!') {
            index += 2
        } else {
            content += string[index]
            index++
        }
    }
    val garbage = Pattern.Garbage(
        content = content,
        depth = parentDepth + 1
    )
    val remainingString = if (index >= string.length) {
        ""
    } else {
        string.substring(startIndex = index + 1)
    }
    return garbage to remainingString
}

fun parseGroups(string: String, parentDepth: Int): Pair<Pattern.Group, String> {
    val depth = parentDepth + 1
    val patterns: MutableList<Pattern> = mutableListOf()
    var currentString = string.substring(startIndex = 1)
    while (currentString.isNotEmpty() && currentString.first() != '}') {
        val (pattern, remainingString) = when (currentString.first()) {
            '<' -> parseGarbage(currentString, depth)
            '{' -> parseGroups(currentString, depth)
            else -> throw IllegalArgumentException()
        }
        patterns.add(pattern)
        currentString = when {
            remainingString.isEmpty() -> remainingString
            remainingString.first() == ',' -> remainingString.substring(startIndex = 1)
            else -> remainingString
        }
    }
    currentString = if (currentString.isNotEmpty()) {
        currentString.substring(startIndex = 1)
    } else {
        currentString
    }
    return Pattern.Group(
        depth = depth,
        patterns = patterns.toList()
    ) to currentString
}

fun getGroupScore(group: Pattern.Group): Int {
    val subGroupsScores = group.patterns
        .filterIsInstance<Pattern.Group>()
        .map { getGroupScore(it) }
    return group.depth + subGroupsScores.sum()
}

fun getGarbageScore(group: Pattern.Group): Int {
    val immediateGarbageScore = group.patterns
        .filterIsInstance<Pattern.Garbage>()
        .sumOf { it.content.length }
    val subGroupsGarbageScoreTotal = group.patterns
        .filterIsInstance<Pattern.Group>()
        .sumOf { getGarbageScore(it) }
    return immediateGarbageScore + subGroupsGarbageScoreTotal
}

fun solveDayNinePartOne() {
    val string = readFile("day_nine.txt").first()
    val (group, _) = parseGroups(string, 0)
    println(getGroupScore(group))
}

fun solveDayNinePartTwo() {
    val string = readFile("day_nine.txt").first()
    val (group, _) = parseGroups(string, 0)
    println(getGarbageScore(group))
}

fun main() {
    solveDayNinePartTwo()
}