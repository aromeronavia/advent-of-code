def main():
    movements = getDeliveryMovementsFromFile('input')
    print getHousesWithAtLeastOnePresent(movements)


def getDeliveryMovementsFromFile(filename):
    return open(filename).readline()


def getHousesWithAtLeastOnePresent(movements):
    x, y = 0, 0
    moves = [(x, y)]    # The first house is already dispatched
    for i in movements:
        if i == '^':
            y += 1
        elif i == 'v':
            y -= 1
        elif i == '>':
            x += 1
        elif i == '<':
            x -= 1

        if (x, y) in moves:
            continue

        moves.append((x, y))

    return len(moves)

if __name__ == "__main__":
    main()
