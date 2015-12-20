def getDimensionsFromFile(file):
    return open(file).readlines()


def main():
    dimensions = getDimensionsFromFile('input')
    print getFeetsOfWrappingPaper(dimensions)


def getFeetsOfWrappingPaper(dimensions):
    feets = 0
    for dimension in dimensions:
        lenghts = [int(x) for x in dimension.split("x")]
        areas = sortDimensions(lenghts)
        feets += ((2 * areas[0]) +
                  (2 * areas[1]) +
                  (2 * areas[2]) +
                  (areas[0]))


    return feets


def sortDimensions(lenghts):
    return sorted([lenghts[0] * lenghts[1],
                   lenghts[1] * lenghts[2],
                   lenghts[2] * lenghts[0]])


if __name__ == "__main__":
    main()
