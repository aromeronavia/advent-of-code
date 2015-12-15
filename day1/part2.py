def main():
    line = getStringFromFile('input')
    print getMinusOneFlor(line)


def getStringFromFile(file):
    return open(file).readline()


def getMinusOneFlor(line):
    floor = 0
    for i in range(len(line)):
        if line[i] == '(':
            floor += 1
        elif line[i] == ')':
            floor -= 1

        if floor == -1:
            return i+1

    return -1

if __name__ == "__main__":
    main()
