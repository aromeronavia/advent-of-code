# quick but awful solution with global var...
variables = {'a': 0, 'b': 0}


class Instruction:
    def __init__(self, raw_instruction):
        split = raw_instruction.split()
        self.statement = split[0]
        if self.statement == 'jmp':
            self.offset = int(split[1])
        else:
            self.register = split[1].rstrip(',')
            if len(split) == 3:
                self.offset = int(split[2])


def solve(instructions):
    pc_counter = 0

    while pc_counter < len(instructions):
        pc_counter += execute(instructions[pc_counter])


def execute(instruction):
    if instruction.statement == 'hlf':
        variables[instruction.register] /= 2
    if instruction.statement == 'tpl':
        variables[instruction.register] *= 3
    if instruction.statement == 'inc':
        variables[instruction.register] += 1
    if instruction.statement == 'jmp':
        return instruction.offset
    if instruction.statement == 'jie':
        if variables[instruction.register] % 2 == 0:
            return instruction.offset

    if instruction.statement == 'jio':
        if variables[instruction.register] == 1:
            return instruction.offset

    return 1


if __name__ == '__main__':
    #  part 1
    variables = {'a': 0, 'b': 0}
    raw_instructions = [i.rstrip('\n') for i in open('input')]
    instructions = [Instruction(i) for i in raw_instructions]
    solve(instructions)
    print variables

    # part 2
    variables = {'a': 1, 'b': 0}
    solve(instructions)
    print variables
