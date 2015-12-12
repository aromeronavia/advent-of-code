day1 = open("day1file", "r").readline()
floor = 0

for i in day1:
    if i == '(':
        floor += 1
    elif i == ')':
        floor -= 1

print floor
