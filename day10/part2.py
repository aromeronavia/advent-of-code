def main():
    print lookAndSay('1113122113')


def lookAndSay(number):
    for i in range(50):
        repeatedDigit = number[0]
        ocurrences = 0
        newNumber = ""
        for digit in number:
            if repeatedDigit != digit:
                newNumber += str(ocurrences) + str(repeatedDigit)
                ocurrences = 1
                repeatedDigit = digit
            else:
                ocurrences += 1

        newNumber += str(ocurrences) + str(repeatedDigit)
        number = newNumber

    return len(number)
main()
