def main():
    string = list("hxbxwxba")
    # string = list("zzzzzzz")
    # string = list("cbbbbfxi")
    for i in range(100):
        if (not hasSequenceOfThreeIncrementalLetters(string) or
               hasConfusingCharacters(string) or
               not hasTwoPairsOfLetters(string)):
            string = incrementString(list(string[::-1]), 0)

    print "".join(string)


def incrementString(stringList, i):
    if i + 1 > len(stringList):
        return stringList[::-1]

    if stringList[i] == "z":
        stringList[i] = "a"
        return incrementString(stringList, i + 1)

        stringList[i] = chr(ord(stringList[i]) + 1)

    return stringList[::-1]


def stringOverflow(character):
    return character == 'z'


def hasSequenceOfThreeIncrementalLetters(string):
    char = ord(string[0])
    i = 1
    lettersInSequence = 1
    while i < len(string):
        if char == ord(string[i]) + 1:
            lettersInSequence += 1
            char = string[i]
            if lettersInSequence == 3:
                return True
        else:
            lettersInSequence = 0

        i += 1

    return False


def hasConfusingCharacters(string):
    if 'i' in string or 'l' in string or 'o' in string:
        return True

    return False


def hasTwoPairsOfLetters(string):
    char = string[0]
    i = 1
    pairs = 0
    while i < len(string):
        if string[i] == char:
            pairs += 1
            i += 1
            try:
                char = string[i]
            except:
                if pairs >= 2:
                    return True
                else:
                    return False

        i += 1

    return pairs >= 2

main()
