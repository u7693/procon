import math

MAX_SIZE = 20 + 1

def prime(x):
    result = [0] * MAX_SIZE
    for i in range(2, int(math.sqrt(x)) + 1):
        while x % i == 0:
            result[i] += 1
            x //= i
        if x == 1:
            break
    if x != 1:
        result[x] += 1
    return result

def solve():
    result = 1
    memo = [0] * MAX_SIZE
    for x in range(1, MAX_SIZE-1):
        l = prime(x)
        for j in range(MAX_SIZE):
            memo[j] = max(memo[j], l[j])
    for i in range(MAX_SIZE):
        result *= i ** memo[i]
    return result

print(solve())
