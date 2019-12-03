from itertools import permutations


class Ingredient:
    def __init__(self, raw_ingredient):
        self.capacity, self.durability, self.flavor, self.texture, \
            self.calories = [int(i) for i in raw_ingredient.split()]

    def __str__(self):
        return "capacity: " + str(self.capacity) +\
            " durability: " + str(self.durability) +\
            " flavor: " + str(self.flavor) +\
            " texture: " + str(self.texture) +\
            " calories: " + str(self.calories)


def get_ingredients(raw_ingredients):
    return [Ingredient(i) for i in raw_ingredients]


def clean_value(value):
    if value < 0:
        return 0
    return value


if __name__ == '__main__':
    raw_ingredients = [i.rstrip('\n') for i in open('input')]
    ingredients = get_ingredients(raw_ingredients)
    numbers = range(101)
    possible_arrays = [i for i in permutations(numbers, 4) if sum(i) == 100]
    sum_capacity = lambda x, y: x * y.capacity
    sum_durability = lambda x, y: x * y.durability
    sum_flavor = lambda x, y: x * y.flavor
    sum_texture = lambda x, y: x * y.texture
    sum_calories = lambda x, y: x * y.calories
    max_score_1 = None
    max_score_2 = None

    for array in possible_arrays:
        capacity = sum(map(sum_capacity, array, ingredients))
        durability = sum(map(sum_durability, array, ingredients))
        flavor = sum(map(sum_flavor, array, ingredients))
        texture = sum(map(sum_texture, array, ingredients))
        calories = sum(map(sum_calories, array, ingredients))

        capacity = clean_value(capacity)
        durability = clean_value(durability)
        flavor = clean_value(flavor)
        texture = clean_value(texture)
        calories = clean_value(calories)

        value = capacity * durability * flavor * texture

        if max_score_1 is None or max_score_1 < value:
            max_score_1 = value

        if max_score_2 is None or (max_score_2 < value and calories == 500):
            max_score_2 = value

    print max_score_1
    print max_score_2
