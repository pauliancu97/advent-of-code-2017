fun getNumOfStepsUntilHalt(jumps: List<Int>): Int {
    val jumpsInstructions = jumps.toMutableList()
    var numOfSteps = 0
    var instructionPointer = 0
    while (instructionPointer in jumpsInstructions.indices) {
        val updatedInstructionPointer = instructionPointer + jumpsInstructions[instructionPointer]
        jumpsInstructions[instructionPointer]++
        instructionPointer = updatedInstructionPointer
        numOfSteps++
    }
    return numOfSteps
}

fun getNumOfStepsUntilHaltExtra(jumps: List<Int>): Int {
    val jumpsInstructions = jumps.toMutableList()
    var numOfSteps = 0
    var instructionPointer = 0
    while (instructionPointer in jumpsInstructions.indices) {
        val updatedInstructionPointer = instructionPointer + jumpsInstructions[instructionPointer]
        jumpsInstructions[instructionPointer] =
            if (jumpsInstructions[instructionPointer] >= 3) {
                jumpsInstructions[instructionPointer] - 1
            } else {
                jumpsInstructions[instructionPointer] + 1
            }
        instructionPointer = updatedInstructionPointer
        numOfSteps++
    }
    return numOfSteps
}

fun readJumps(path: String) =
    readFile(path)
        .map { line ->
            line
                .filterNot { it == '\n' }
                .toInt()
        }

fun solveDayFivePartOne() {
    val jumps = readJumps("day_five.txt")
    print(getNumOfStepsUntilHalt(jumps))
}

fun solveDayFivePartTwo() {
    val jumps = readJumps("day_five.txt")
    print(getNumOfStepsUntilHaltExtra(jumps))
}

fun main() {
    solveDayFivePartTwo()
}