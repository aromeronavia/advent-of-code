from constants import *


class Solver:
    def __init__(self, lights):
        self.lights = lights

    def executeInstruction(self, instruction):
        action = instruction.action
        for i in range(instruction.origin.getX(),
                       instruction.departure.getX() + 1):
            for j in range(instruction.origin.getY(),
                           instruction.departure.getY() + 1):
                self.lights.matrix[i][j] += self.getValueFromAction(action,
                                            self.lights.matrix[i][j])
                if self.lights.matrix[i][j] < 0:
                    self.lights.matrix[i][j] = 0

    def getValueFromAction(self, action, light):
        if action == 'on':
            return ON_VALUE
        elif action == 'off':
            return OFF_VALUE
        else:
            return TOGGLE_VALUE

    def toggleLight(self, light):
        return light * -1


class Lights:
    def __init__(self):
        self.matrix = self.createLights()

    # Matrix filled with zeros
    def createLights(self):
        return [[0 for i in range(1000)] for j in range(1000)]


class Coord:
    def __init__(self, coord):
        self.x = int(coord[0])
        self.y = int(coord[1])

    def getX(self):
        return self.x

    def getY(self):
        return self.y


class Instruction:
    def __init__(self, instructionString):
        instructionSplitted = self.getInstructionSplitted(instructionString)
        self.action = instructionSplitted[INSTRUCTION]
        self.origin = Coord([i for i in
                            instructionSplitted[ORIGIN_COORDS]
                            .split(',')])
        self.departure = Coord([i for i in
                               instructionSplitted[DEPARTURE_COORDS]
                               .split(',')])

    def getInstructionSplitted(self, instructionString):
        return self.splitBySpaces(instructionString)

    def splitBySpaces(self, instruction):
        return [i for i in instruction.split(' ')]

    def __repr__(self):
        return self.action, self.origin.getX() + ',' + self.origin.getY(),\
            self.departure.getX() + ',' + self.departure.getY()


def main():
    instructions = getLinesFromFile('input')
    matrix = solveInput(instructions)
    print getBrighnessSum(matrix)


def getBrighnessSum(matrix):
    count = 0
    for i in range(len(matrix)):
        for j in range(len(matrix)):
                count += matrix[i][j]

    return count


def getLinesFromFile(input):
    return [line.rstrip('\n') for line in open('input', 'r').readlines()]


def solveInput(instructions):
    solver = Solver(Lights())
    for instructionString in instructions:
        solver.executeInstruction(getInstructionFromString(instructionString))
    return solver.lights.matrix


def getInstructionFromString(instructionString):
    return Instruction(instructionString)


main()
