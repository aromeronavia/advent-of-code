import json


def sumJson(data):
    total = 0
    if isinstance(data, dict):
        vals = data.values()
        if 'red' in vals:
            return 0
        else:
            total += iterateJsonRecursively(vals)
    elif isinstance(data, list):
        total += iterateJsonRecursively(data)
    return total


def iterateJsonRecursively(sequence):
    valuesSum = 0
    for value in sequence:
        if isinstance(value, int):
            valuesSum += value
        elif isinstance(value, (list, dict)):
            valuesSum += sumJson(value)
    return valuesSum


if __name__ == '__main__':
    inputJson = open('input')
    data = json.load(inputJson)
    print("Part 2:", sumJson(data))
