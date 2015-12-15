def main():
    line = getStringFromFile('input')
    print getFloor(line)


def getStringFromFile(file):
    return open(file).readline()


def getFloor(line):
    floor = 0
    for i in line:
        if i == '(':
            floor += 1
        elif i == ')':
            floor -= 1

    return str(floor)


main()
