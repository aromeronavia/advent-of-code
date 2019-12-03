class Reindeer:
    def __init__(self, raw_reindeer):
        self.name, self.speed, self.run_time, self.rest_time = \
            raw_reindeer.split()
        self.speed = int(self.speed)
        self.run_time = int(self.run_time)
        self.rest_time = int(self.rest_time)

    def run(self, seconds):
        distance = 0
        get_distance = self.get_distance
        while seconds >= 0:
            distance += get_distance(self.speed, self.run_time, seconds)
            seconds -= self.run_time
            seconds -= self.rest_time

        return distance

    def get_distance(self, speed, time, seconds):
        if seconds - time >= 0:
            return (speed * self.run_time)
        else:
            return (speed * seconds)


def get_winner(reindeers):
    winner = None
    seconds = 0
    while seconds < 2504:
        for reindeer in reindeers:
            reindeer_time = reindeer.run(seconds)
            if winner is None or winner < reindeer_time:
                winner = reindeer_time

        seconds += 1
    return winner


if __name__ == '__main__':
    reindeers = [i.rstrip('\n') for i in open('input')]
    reindeers = [Reindeer(i) for i in reindeers]
    winner = get_winner(reindeers)
    print winner
