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
        print literals
        if '\\' in string:
            characters += 2
        return



    return str(int(literals) - int(characters))

main()
