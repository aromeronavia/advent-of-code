from itertools import combinations


if __name__ == '__main__':
    containers = [43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17,
                  17, 44, 36, 31, 46, 9, 27, 38]

    num_combinations = 0
    minimum_combinations = 0

    for i in range(len(containers)):
        num_combinations += sum([1 for x in combinations(containers, i) if sum(list(x)) == 150])

    minimum_combinations = sum([1 for x in combinations(containers, 4) if sum(list(x)) == 150])

    print num_combinations
    print minimum_combinations
