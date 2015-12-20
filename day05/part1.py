'''
A nice string contains:
    1) At least three vowels
    2) At least one letter that appears twice in a row
    3) It does not contains: ['ab', 'cd', 'pq', 'xy']
'''


def niceString(string):
    vowelsArray = ['a', 'e', 'i', 'o', 'u']
    invalidStrings = ['ab', 'cd', 'pq', 'xy']
    pastIterationLetter = '0'
    characters = []
    twiceInARow = False
    vowelsInString = 0

    for i in string:
        # 3
        if pastIterationLetter + i in invalidStrings:
            pastIterationLetter = i
            return False

        # 2
        if pastIterationLetter == i:
            twiceInARow = True

        characters.append(i)
        pastIterationLetter = i

    # 1
    for i in string:
        if i in vowelsArray:
            vowelsInString += 1

    if vowelsInString >= 3:
        return twiceInARow

    return False


def getNiceStrings(inputLines):
    niceStrings = 0
    for i in inputLines:
        if niceString(i):
            niceStrings += 1

    return niceStrings


def getInputLines(filename):
    return open(filename, 'r').readlines()


def main():
    filename = 'input'
    inputLines = getInputLines(filename=filename)
    print getNiceStrings(inputLines)

main()
