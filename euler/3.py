import math

def solve(x):
    result = 0
    for i in range(2, int(math.sqrt(x)) + 10):
        while x % i == 0:
            result = max(result, i)
            x //= i
        if (x == 1):
            break
    if x != 1:
        result = max(result, x)
    return result

assert solve(13195) == 29
print(solve(600851475143))
