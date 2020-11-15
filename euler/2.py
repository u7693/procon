def solve(x):
    a = 1
    b = 2
    result = 0
    while b <= x:
        if b % 2 == 0:
            result += b
        c = a + b
        a = b
        b = c
    return result

assert solve(34) == 44
assert solve(89) == 44
print(solve(4000000))
