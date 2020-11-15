def solve(x):
    result = 0
    for i in range(1, x):
        if i % 3 == 0 or i % 5 == 0:
            result += i
    return result

assert solve(10) == 23
print(solve(1000))
