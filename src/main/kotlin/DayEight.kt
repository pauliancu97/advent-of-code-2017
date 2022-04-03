import kotlin.math.max

enum class RegisterInstructionType(private val opCode: String, val sign: Int) {
    Increment("inc", 1),
    Decrement("dec", -1);

    companion object {
        fun getRegisterInstructionType(string: String): RegisterInstructionType? =
            RegisterInstructionType.values()
                .firstOrNull { it.opCode == string }
    }
}

enum class ConditionalType(private val value: String) {
    Less("<"),
    Greater(">"),
    LessOrEqual("<="),
    GreaterOrEqual(">="),
    Equal("=="),
    NotEqual("!=");

    companion object {
        fun getConditionalType(string: String): ConditionalType? =
            ConditionalType.values()
                .firstOrNull { it.value == string }
    }
}


data class RegisterInstruction(
    val targetRegister: String,
    val instructionType: RegisterInstructionType,
    val updateValue: Int,
    val testRegister: String,
    val conditionalType: ConditionalType,
    val conditionalValue: Int
)

typealias RegistersState = Map<String, Int>
typealias RegisterProgram = List<RegisterInstruction>

val REGISTER_INSTRUCTION_REGEX = """(\w+) (inc|dec) ([+-]?\d+) if (\w+) (>|<|>=|<=|==|!=) ([+-]?\d+)""".toRegex()

fun String.toRegisterInstruction(): RegisterInstruction? {
    val match = REGISTER_INSTRUCTION_REGEX.matchEntire(this)
    return match
        ?.let { matchResult ->
            val targetRegister = matchResult.groupValues[1]
            val registerInstructionType = RegisterInstructionType.getRegisterInstructionType(
                matchResult.groupValues[2]
            ) ?: return@let null
            val updateValue = matchResult.groupValues[3].toIntOrNull() ?: return@let null
            val testRegister = matchResult.groupValues[4]
            val conditionalType = ConditionalType.getConditionalType(
                matchResult.groupValues[5]
            ) ?: return@let null
            val conditionalValue = matchResult.groupValues[6].toIntOrNull() ?: return@let null
            RegisterInstruction(
                targetRegister = targetRegister,
                instructionType = registerInstructionType,
                updateValue = updateValue,
                testRegister = testRegister,
                conditionalType = conditionalType,
                conditionalValue = conditionalValue
            )
        }
}

fun getUpdatedRegisterState(registersState: RegistersState, instruction: RegisterInstruction): RegistersState {
    val testRegisterValue = registersState[instruction.testRegister] ?: return registersState
    val isConditionMet = when (instruction.conditionalType) {
        ConditionalType.Less -> testRegisterValue < instruction.conditionalValue
        ConditionalType.LessOrEqual -> testRegisterValue <= instruction.conditionalValue
        ConditionalType.Greater -> testRegisterValue > instruction.conditionalValue
        ConditionalType.GreaterOrEqual -> testRegisterValue >= instruction.conditionalValue
        ConditionalType.Equal -> testRegisterValue == instruction.conditionalValue
        ConditionalType.NotEqual -> testRegisterValue != instruction.conditionalValue
    }
    return if (isConditionMet) {
        registersState.updated(instruction.targetRegister) { value ->
            value + instruction.instructionType.sign * instruction.updateValue
        }
    } else {
        registersState
    }
}

fun getRegistersState(program: RegisterProgram): RegistersState =
    program
        .flatMap { listOf(it.targetRegister, it.testRegister) }
        .distinct()
        .associateWith { 0 }

fun getFinalRegistersState(program: RegisterProgram): RegistersState {
    val initialRegistersState = getRegistersState(program)
    return program
        .fold(initialRegistersState) { state, instruction -> getUpdatedRegisterState(state, instruction) }
}

fun getHighestMaxValue(program: RegisterProgram): Int {
    var state = getRegistersState(program)
    var maxValue = 0
    for (instruction in program) {
        state = getUpdatedRegisterState(state, instruction)
        val currentMaxValue = state
            .values
            .maxOrNull() ?: 0
        maxValue = max(maxValue, currentMaxValue)
    }
    return maxValue
}

fun readProgram(path: String): RegisterProgram =
    readFile(path)
        .mapNotNull { it.toRegisterInstruction() }

fun solveDayEightPartOne() {
    val program = readProgram("day_eight.txt")
    val state = getFinalRegistersState(program)
    val result = state.values.maxOrNull() ?: 0
    println(result)
}

fun solveDayEightPartTwo() {
    val program = readProgram("day_eight.txt")
    val result = getHighestMaxValue(program)
    println(result)
}

fun main() {
    solveDayEightPartTwo()
}