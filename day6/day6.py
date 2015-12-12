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
                self.lights.matrix[i][j] = self.getValueFromAction(action,
                                           self.lights.matrix[i][j])

    def getValueFromAction(self, action, light):
        if action == 'on':
            return LIGHT_ON
        elif action == 'off':
            return LIGHT_OFF
        else:
            return toggleLight(light)


class Lights:
    def __init__(self):
        self.matrix = self.createLights()

    # Matrix filled with zeros
    def createLights(self):
        return [[-1 for i in range(1000)] for j in range(1000)]


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


def toggleLight(light):
        return light * -1


def main():
    instructions = getLinesFromFile('input')
    matrix = solveInput(instructions)
    print countLightsOn(matrix)


def countLightsOn(matrix):
    count = 0
    for i in range(len(matrix)):
        for j in range(len(matrix)):
            if matrix[i][j] == LIGHT_ON:
                count += 1

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
