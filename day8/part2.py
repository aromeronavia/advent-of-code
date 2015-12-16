def main():
    stringLines = getLinesFromFile('input')
    solution = getSolution(stringLines[:])
    print solution - sum([len(line) for line in stringLines])


def getOriginalSizes(stringLines):
    pass


def getLinesFromFile(input):
    return [str(line.rstrip('\n'))
            for line in open(input, 'r').readlines()]


def getSolution(stringLines):
    encodedChars, characters = 0, 0
    for string in stringLines:
        for i in range(len(string)):
            if string[i] == '\\' or string[i] == '\"':
                encodedChars += 2
            else:
                encodedChars += 1

        encodedChars += 2

    return encodedChars

main()
