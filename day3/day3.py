day3file = open("day3file", "r").readline()
x, y = 0, 0
moves = [(x, y)]

for i in day3file:
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

print len(moves)
