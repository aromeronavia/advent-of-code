from itertools import count
from hashlib import md5


def main():
    for x in count(1):
        string = 'iwrupvqb' + str(x)
        if md5(string.encode('utf-8')).hexdigest()[:5] == '00000':
            print(x)
            break

main()
