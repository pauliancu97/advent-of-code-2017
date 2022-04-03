data class InputProgram(
    val name: String,
    val weight: Int,
    val subProgramsNames: List<String> = emptyList()
)


data class Program(
    val name: String,
    val weight: Int,
    val subPrograms: List<Program> = emptyList()
)

private val INPUT_PROGRAM_REGEX = """(\w+) \((\d+)\)(?: -> )?""".toRegex()


fun getInputProgram(string: String): InputProgram? {
    val matchResult = INPUT_PROGRAM_REGEX.find(string)
    return matchResult
        ?.let { match ->
            val name = match.groupValues[1]
            val weight = match.groupValues[2].toIntOrNull() ?: return@let null
            val subProgramsNamesSubstring = string.substring(startIndex = match.range.last + 1)
            val subProgramNames = if (subProgramsNamesSubstring.isNotEmpty()) {
                subProgramsNamesSubstring.split(", ")
            } else {
                emptyList()
            }
            InputProgram(
                name = name,
                weight = weight,
                subProgramsNames = subProgramNames
            )
        }
}

fun readInputPrograms(path: String): List<InputProgram> =
    readFile(path)
        .mapNotNull { getInputProgram(it) }


fun getRootProgram(inputPrograms: List<InputProgram>): InputProgram {
    val subProgramsNames = inputPrograms
        .flatMap { it.subProgramsNames }
        .toSet()
    return inputPrograms
        .first { it.name !in subProgramsNames }
}

fun getProgramHelper(inputProgram: InputProgram, inputPrograms: Map<String, InputProgram>): Program {
    val subPrograms = inputProgram.subProgramsNames
        .mapNotNull { subProgramName ->
            inputPrograms[subProgramName]
                ?.let { getProgramHelper(it, inputPrograms) }
        }
    return Program(
        name = inputProgram.name,
        weight = inputProgram.weight,
        subPrograms = subPrograms
    )
}

fun getProgram(inputPrograms: List<InputProgram>): Program {
    val rootInputProgram = getRootProgram(inputPrograms)
    val inputProgramsByName = inputPrograms.associateBy { it.name }
    return getProgramHelper(rootInputProgram, inputProgramsByName)
}

fun solveDaySevenPartOne() {
    val inputPrograms = readInputPrograms("day_seven.txt")
    print(getRootProgram(inputPrograms).name)
}

fun getProgramWeight(program: Program): Int {
    val subProgramsWeights = program.subPrograms
        .map { getProgramWeight(it) }
    return program.weight + subProgramsWeights.sum()
}

fun getAdjustedWeightHelper(program: Program, programWeight: Int, weightSiblings: Int): Int {
    val weightsWithSubPrograms = program.subPrograms
        .groupBy { getProgramWeight(it) }
    return if (weightsWithSubPrograms.size > 1) {
        val unbalancedProgramWeight = weightsWithSubPrograms
            .toList()
            .first { (_, programs) -> programs.size == 1 }
            .first
        val unbalancedSubProgram = weightsWithSubPrograms
            .toList()
            .first { (weight, _) -> weight == unbalancedProgramWeight }
            .second
            .first()
        val updatedWeightSiblings = weightsWithSubPrograms
            .keys
            .first { it != unbalancedProgramWeight }
        getAdjustedWeightHelper(unbalancedSubProgram, unbalancedProgramWeight, updatedWeightSiblings)
    } else {
        val adjustment = weightSiblings - programWeight
        program.weight + adjustment
    }
}

fun getAdjustedWeight(program: Program): Int {
    val weightsWithSubPrograms = program.subPrograms
        .groupBy { getProgramWeight(it) }
    val unbalancedProgramWeight = weightsWithSubPrograms
        .toList()
        .first { (_, programs) -> programs.size == 1 }
        .first
    val unbalancedSubProgram = weightsWithSubPrograms
        .toList()
        .first { (weight, _) -> weight == unbalancedProgramWeight }
        .second
        .first()
    val updatedWeightSiblings = weightsWithSubPrograms
        .keys
        .first { it != unbalancedProgramWeight }
    return getAdjustedWeightHelper(unbalancedSubProgram, unbalancedProgramWeight, updatedWeightSiblings)
}

fun solveDaySevenPartTwo() {
    val inputPrograms = readInputPrograms("day_seven.txt")
    val program = getProgram(inputPrograms)
    println(getAdjustedWeight(program))
}

fun main() {
    solveDaySevenPartTwo()
}