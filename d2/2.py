import re

with open('./2/input.txt', 'r') as input:
    print("Part 1")

    total = 0
    for line in input:
        prev = -1
        ascending = None
        ok = True
        for n in line[:-1].split(' '):
            if prev == -1:
                prev = int(n)
                continue
            if ascending == None:
                if prev > int(n):
                    ascending = False
                elif prev < int(n):
                    ascending = True
                else:
                    ok = False
                    continue
            else:
                if prev > int(n) and ascending:
                    ok = False
                if prev < int(n) and not ascending:
                    ok = False

            if (1 > abs(prev - int(n))) or (abs(prev - int(n)) > 3):
                ok = False

            prev = int(n)

        if ok:
            total += 1

    print(total)

    def is_ok(list):
        prev = -1
        ascending = None

        for i, n in enumerate(list):
            if prev == -1:
                prev = n
                continue
            if ascending == None:
                if prev > n:
                    ascending = False
                elif prev < n:
                    ascending = True
                else:
                    return False
            else:
                if prev > n and ascending:
                    return False
                if prev < n and not ascending:
                    return False

            if (1 > abs(prev - n)) or (abs(prev - n) > 3):
                return False

            prev = n

        return True

with open('./2/input.txt', 'r') as input:
    print("Part 2")

    total = 0
    for line in input:
        numbers = [int(n) for n in line[:-1].split(' ')]

        ok = is_ok(numbers)
        if ok:
            total += 1
        else:
            for a in range(len(numbers)):
                ns = numbers[0:a] + numbers[a+1:]
                
                if is_ok(ns):
                    total += 1
                    break

            # numbers.pop(i-1)
            # ok, _ = is_ok(numbers)
            # if ok:
            #     total += 1

    print(total)