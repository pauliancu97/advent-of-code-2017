fun isPasswordValid(password: String): Boolean {
    val words = password.split(" ")
    val wordSet: MutableSet<String> = mutableSetOf()
    for (word in words) {
        if (word in wordSet){
            return false
        } else {
            wordSet.add(word)
        }
    }
    return true
}

fun isPasswordSecure(password: String): Boolean {
    val anagrams = password
        .split(" ")
        .map { it.toList().sorted() }
    val anagramsSet: MutableSet<List<Char>> = mutableSetOf()
    for (anagram in anagrams) {
        if (anagram in anagramsSet) {
            return false
        } else {
            anagramsSet.add(anagram)
        }
    }
    return true
}


fun getNumValidPasswords(passwords: List<String>) =
    passwords.count { isPasswordValid(it) }


fun getNumOfSecurePasswords(passwords: List<String>) =
    passwords.count { isPasswordSecure(it) }

fun solvePartOne() {
    val passwords = readFile("day_four.txt")
    print(getNumValidPasswords(passwords))
}

fun solvePartTwo() {
    val passwords = readFile("day_four.txt")
    print(getNumOfSecurePasswords(passwords))
}

fun main() {
    solvePartTwo()
}