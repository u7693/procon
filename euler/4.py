def check(x):
    for i in range(100, 1000):
        if x % i == 0 and len(str(x // i)) == 3:
            return (True, i)
    return (False, -1)

def solve():
    for i in reversed(range(100, 1000)):
        t = int(str(i) + ''.join(list(reversed(str(i)))))
        (x, y) = check(t)
        if x:
            print(str(y) + " * " + str(t // y))
            return t

print(solve())
