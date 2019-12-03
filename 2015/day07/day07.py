import operator


class Wrapper:
    def __init__(self, instructions, variables):
        self.instructions = instructions
        self.variables = variables

    def get_signal(self, signal):
        try:
            return int(signal)  # we have a concrete value
        except:
            pass

        if signal not in self.variables:
            instruction = self.instructions[signal]
            instruction = instruction.split()
            if len(instruction) == 1:
                value = self.get_signal(instruction[0])

            else:
                if instruction[0] == 'NOT':
                    value = 65535 - self.get_signal(instruction[-1])

                elif instruction[1] == 'AND':
                    value = self.get_signal(instruction[0]) & \
                        self.get_signal(instruction[-1])

                elif instruction[1] == 'OR':
                    value = self.get_signal(instruction[0]) | \
                        self.get_signal(instruction[-1])

                elif instruction[1] == 'LSHIFT':
                    value = self.get_signal(instruction[0]) << \
                        self.get_signal(instruction[-1])

                elif instruction[1] == 'RSHIFT':
                    value = self.get_signal(instruction[0]) >> \
                        self.get_signal(instruction[-1])


            self.variables[signal] = value

        return self.variables[signal]


def read_lines_from_file(filename):
    return [i.rstrip('\n') for i in open(filename).readlines()]


def main():
    #Part 1
    lines = read_lines_from_file('input')
    instructions, variables = get_instructions_and_vars(lines)
    wrapper = Wrapper(instructions, variables)
    a = wrapper.get_signal('a')
    print a

    # Part 2
    instructions, variables = get_instructions_and_vars(lines)
    wrapper = Wrapper(instructions, variables)
    wrapper.variables['b'] = a
    a = wrapper.get_signal('a')
    print a


def get_instructions_and_vars(lines):
    instructions = {}
    var_map = {}
    for line in lines:
        value, key = line.split(' -> ')
        instructions[key] = value

    return instructions, var_map

if __name__ == '__main__':
    main()
