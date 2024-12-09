import re

a = []
b = []

with open('./1/input.txt', 'r') as input:
    print("Part 1")

    for line in input:
        f, s = re.findall(r'^([0-9]+)\s*([0-9]+)$', line)[0]

        a.append(int(f))
        b.append(int(s))

a.sort()
b.sort()

total = 0
for i in range(len(a)):
    total += abs(a[i] - b[i])

print(total)


print ("\nPart2\n")




total = 0
for n in a:
    t = 0
    for m in b:
        if m == n:
            t += 1

    total += n*t

print(total)
