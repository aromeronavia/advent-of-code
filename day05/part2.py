def main():
    strings = getStringsFromFile('input')
    print niceStringsFromArray(strings)


def niceStringsFromArray(strings):
    numberOfNiceStrings = 0
    for string in strings:
        arrayOfPairs = getPairsOfTwoLetters(string)
        pairTwice = hasPairTwice(arrayOfPairs[:])
        charBetweenTwo = hasCharBetweenTwo(string)
        if pairTwice and charBetweenTwo:
            numberOfNiceStrings += 1

    return numberOfNiceStrings


def hasPairTwice(arrayOfPairs):
    while len(arrayOfPairs) >= 2:
        pair = arrayOfPairs.pop()
        aux = arrayOfPairs.pop()
        print pair, aux, arrayOfPairs
        if pair in arrayOfPairs:
            return True

        arrayOfPairs.append(aux)

    return False


def hasCharBetweenTwo(string):
    for i in range(len(string)-2):
        if string[i] == string[i+2]:
            return True

    return False


def getPairsOfTwoLetters(string):
    return [string[i] + string[i+1]
            for i in range(len(string) - 1)]

def getStringsFromFile(filename):
    return [i.rstrip('\n') for i in open(filename).readlines()]


if __name__ == "__main__":
    main()
