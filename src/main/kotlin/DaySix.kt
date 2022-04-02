fun getAfterCycle(banks: List<Int>): List<Int> {
    val mutableBanks = banks.toMutableList()
    val (maxIndex, maxNumBlocks) = banks
        .withIndex()
        .sortedWith(compareBy({ -it.value }, { it.index }))
        .first()
    mutableBanks[maxIndex] = 0
    for (index in 1..maxNumBlocks) {
        val listIndex = (maxIndex + (index % banks.size)) % banks.size
        mutableBanks[listIndex]++
    }
    return mutableBanks.toList()
}

fun getNumStepsUntilConfigurationRepeat(banks: List<Int>): Int {
    val configurations: MutableSet<List<Int>> = mutableSetOf(banks)
    var currentBanks = banks
    var hasRepeatedConfigurations = false
    while (!hasRepeatedConfigurations) {
        currentBanks = getAfterCycle(currentBanks)
        hasRepeatedConfigurations = currentBanks in configurations
        if (!hasRepeatedConfigurations) {
            configurations.add(currentBanks)
        }
    }
    return configurations.size
}

fun getFirstRepeatedConfiguration(banks: List<Int>): List<Int> {
    val configurations: MutableSet<List<Int>> = mutableSetOf(banks)
    var currentBanks = banks
    var hasRepeatedConfigurations = false
    while (!hasRepeatedConfigurations) {
        currentBanks = getAfterCycle(currentBanks)
        hasRepeatedConfigurations = currentBanks in configurations
        if (!hasRepeatedConfigurations) {
            configurations.add(currentBanks)
        }
    }
    return currentBanks
}

fun getNumStepsUntilInitialConfiguration(banks: List<Int>): Int {
    var currentBanks = getAfterCycle(banks)
    var numOfSteps = 1
    while (currentBanks != banks) {
        currentBanks = getAfterCycle(currentBanks)
        numOfSteps++
    }
    return numOfSteps
}

fun getSizeOfLoop(banks: List<Int>): Int {
    val repeatedConfig = getFirstRepeatedConfiguration(banks)
    return getNumStepsUntilInitialConfiguration(repeatedConfig)
}

fun solveDaySixPartOne() {
    println(getNumStepsUntilConfigurationRepeat(listOf(4, 10, 4, 1, 8, 4, 9, 14, 5, 1, 14, 15, 0, 15, 3, 5)))
}


fun solveDaySixPartTwo() {
    println(getSizeOfLoop(listOf(4, 10, 4, 1, 8, 4, 9, 14, 5, 1, 14, 15, 0, 15, 3, 5)))
}

fun main() {
    solveDaySixPartTwo()
}