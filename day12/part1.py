import re


def main():
    fileInput = readfile('input')
    pattern = re.compile(r"[+-]?\d+")
    matches = pattern.findall(fileInput)
    print sum([int(i) for i in matches])


def readfile(input):
    return open(input).readline()

main()
