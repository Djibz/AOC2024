import re

with open('./3/input.txt', 'r') as input:
    print("Part 1")

    total = 0
    for l in input:
        total += sum([int(a) * int(b) for a, b in re.findall('mul\((\d+),(\d+)\)', l)])

    print(total)

with open('./3/input.txt', 'r') as input:
    print("Part 2")

    total = 0
    do = True
    for l in input:
        muls = re.findall('mul\((\d+),(\d+)\)|(do)\(\)|(don\'t)\(\)', l)
    
        for a, b, y, n in muls:
            if a and b and (do):
                total += int(a) * int(b)
            if y == "do":
                do = True
            if n == "don't":
                do = False

    print(total)