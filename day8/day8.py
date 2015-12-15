import re


def main():
    stringLines = getLinesFromFile('input')
    solution = getSolution(stringLines)
    print solution


def getLinesFromFile(input):
    return [line.rstrip('\n')
            for line in open(input, 'r').readlines()]


def getSolution(stringLines):
    literals, characters = 0, 0
    for string in stringLines:
        literals += len(string)
        characters += len(re.sub(r"\\x[0-9A-Fa-f][0-9A-Fa-f]|\\\\|\\\"",
                                 "0", string.decode('raw_unicode_escape'))) - 2

    return str(int(literals) - int(characters))

main()
