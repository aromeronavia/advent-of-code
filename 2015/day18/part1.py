SIZE_OF_MATRIX = 100


def getLinesFromFile(filename):
    return [list(i.rstrip('\n')) for i in open(filename).readlines()]


def main():
    lightsMatrix = getLinesFromFile('input')
    print solve(lightsMatrix)


def solve(matrix):
    newMatrix = [row[:] for row in matrix]
    for i in range(100):
        for i in range(SIZE_OF_MATRIX):
            for j in range(SIZE_OF_MATRIX):
                if currentIsOn(matrix, i, j):
                    if (neighboursOn(matrix, i, j) in [2, 3]):
                        setOn(newMatrix, i, j)
                    else:
                        setOff(newMatrix, i, j)
                else:
                    if neighboursOn(matrix, i, j) == 3:
                        setOn(newMatrix, i, j)
                    else:
                        setOff(newMatrix, i, j)

        matrix = newMatrix
        newMatrix = [row[:] for row in matrix]

    return getAllLightsOn(newMatrix)


def getAllLightsOn(matrix):
    count = 0
    for i in range(SIZE_OF_MATRIX):
        for j in range(SIZE_OF_MATRIX):
            if currentIsOn(matrix, i, j):
                count += 1

    return count


def neighboursOn(matrix, i, j):
    neighbours = 0
    permutations = [[i, j+1],
                    [i+1, j+1],
                    [i+1, j],
                    [i+1, j-1],
                    [i, j-1],
                    [i-1, j-1],
                    [i-1, j],
                    [i-1, j+1]]

    for x, y in permutations:
        if 0 <= x <= SIZE_OF_MATRIX - 1 and 0 <= y <= SIZE_OF_MATRIX - 1:
            if matrix[x][y] == '#':
                    neighbours += 1

    return neighbours


def setOn(matrix, i, j):
    matrix[i][j] = '#'


def setOff(matrix, i, j):
    if not ((i == 0 and j == 0) or
            (i == 0 and j == SIZE_OF_MATRIX - 1) or
            (i == SIZE_OF_MATRIX - 1 and j == 0) or
            (i == SIZE_OF_MATRIX - 1and j == SIZE_OF_MATRIX - 1)):
        matrix[i][j] = '.'


def currentIsOn(matrix, i, j):
    try:
        return matrix[i][j] == '#'
    except:
        return False

main()
