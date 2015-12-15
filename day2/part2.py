def getDimensionsFromFile(file):
    return open(file).readlines()


def main():
    dimensions = getDimensionsFromFile('input')
    print getFeetsOfRibbon(dimensions)


def getFeetsOfRibbon(dimensions):
    feets = 0
    for dimension in dimensions:
        lenghts = sorted([int(x) for x in dimension.split("x")])
        feets += lenghts[0]*2 + lenghts[1]*2 +\
            lenghts[0] * lenghts[1] * lenghts[2]

    return feets

if __name__ == "__main__":
    main()
