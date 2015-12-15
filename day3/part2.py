SANTA = 0
BOT = 1


def main():
    movements = getDeliveryMovementsFromFile('input')
    print getHousesWithSantaAndBot(movements)


def getDeliveryMovementsFromFile(filename):
    return open(filename).readline()


def getHousesWithSantaAndBot(movements):
    moves = [(0, 0)]    # The first house is already dispatched
    guysPositions = [[0, 0], [0, 0]]
    print guysPositions
    guy = SANTA
    for move in movements:
        guysPositions[guy] = (evaluateMove(move, guysPositions[guy]))
        if tuple(guysPositions[guy]) not in moves:
            moves.append(tuple(guysPositions[guy]))

        guy = (guy + 1) % 2

    return len(moves)


def evaluateMove(move, coords):
    if move == '^':
        coords[1] += 1
    elif move == 'v':
        coords[1] -= 1
    elif move == '>':
        coords[0] += 1
    elif move == '<':
        coords[0] -= 1

    return [coords[0], coords[1]]


if __name__ == "__main__":
    main()
