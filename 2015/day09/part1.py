from collections import defaultdict
from itertools import permutations


def main():
    distances = [i.rstrip('\n') for i in open('input').readlines()]
    routes = get_routes(distances)
    print shortest_route(routes)
    print longest_route(routes)


def shortest_route(routes):
    all_routes = permutations(routes.keys())
    max_distance = None
    for route in all_routes:
        distance = get_distance(route, routes)
        if max_distance is None or distance < max_distance:
            max_distance = distance

    return max_distance


def longest_route(routes):
    all_routes = permutations(routes.keys())
    max_distance = None
    for route in all_routes:
        distance = get_distance(route, routes)
        if max_distance is None or distance > max_distance:
            max_distance = distance

    return max_distance


def get_distance(route, routes):
    distance_between = lambda a, b: routes[a][b]
    return sum(map(distance_between, route[:-1], route[1:]))


def get_routes(distances):
    routes = defaultdict(dict)
    for distance in distances:
        distance = distance.split()
        routes[distance[0]][distance[1]] = int(distance[-1])
        routes[distance[1]][distance[0]] = int(distance[-1])


    return routes

if __name__ == '__main__':
    main()
