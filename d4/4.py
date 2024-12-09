import numpy as np

def count_xmas(i, j, m):
    string = 'XMAS'
    if m[i, j] != 'X': return 0

    total = 0

    for a in [-1, 0, 1]:
        for b in [-1, 0, 1]:
            ok = True
            for k, l in enumerate(string):
                x = i + (a*k)
                y = j + (b*k)
                
                if x < 0 or y < 0 or x >= m.shape[0] or y >= m.shape[1]:
                    ok = False
                    break
                
                if m[x, y] != l:
                    ok = False

            if ok:
                total += 1

    return total

def count_x_mas(i, j, m):
    if m[i, j] != 'A': return 0

    if m[i-1, j+1] == 'M' and m[i-1, j-1] == 'M' and m[i+1, j+1] == 'S' and m[i+1, j-1] == 'S':
        return 1

    return 0


print("Part 1")

m = np.genfromtxt("./4/input.txt", delimiter=1, dtype='unicode')

total1 = 0
total2 = 0
for i in range(m.shape[0]):
    for j in range(m.shape[1]):
        total1 += count_xmas(i, j, m)

print(total1)

for _ in range(4):
    for i in range(1, m.shape[0]-1):
        for j in range(1, m.shape[1]-1):
            total2 += count_x_mas(i, j, m)

    m = np.rot90(m)

print(total2)