from itertools import permutations
from collections import defaultdict


def get_happiness_from_input(filename):
    return [i.rstrip('\n') for i in open(filename)]


def get_relations_happiness(happiness_list):
    relation = defaultdict(dict)
    for happiness in happiness_list:
        person1, person2, sign, value = happiness.split(' ')
        if sign == 'gain':
            sign = 1
        else:
            sign = -1

        relation[person1][person2] = sign * int(value)

    return relation


def get_relations_happiness_with_me(happiness_list):
    relation = defaultdict(dict)
    for happiness in happiness_list:
        person1, person2, sign, value = happiness.split(' ')
        if sign == 'gain':
            sign = 1
        else:
            sign = -1

        relation[person1][person2] = sign * int(value)
        relation[person1]['Alberto'] = 0
        relation['Alberto'][person2] = 0

    return relation

def optimal_arrangement(relations):
    possible_arrangements = permutations(relations.keys())
    happiness = None
    for arrangement in possible_arrangements:
        value = get_sum_of_happiness(arrangement, relations)
        if happiness is None or happiness < value:
            happiness = value

    return happiness

def get_sum_of_happiness(arrangement, relations):
    i = 0
    happiness = 0
    while i < len(arrangement):
        person1 = arrangement[i]
        try:
            person2 = arrangement[i+1]
        except:
            person2 = arrangement[0]

        happiness += relations[person1][person2]
        happiness += relations[person2][person1]

        i += 1

    return happiness

if __name__ == '__main__':
    # part 1
    happiness = get_happiness_from_input('input')
    relations = get_relations_happiness(happiness)
    print optimal_arrangement(relations)

    # part 2
    relations = get_relations_happiness_with_me(happiness)
    print optimal_arrangement(relations)
