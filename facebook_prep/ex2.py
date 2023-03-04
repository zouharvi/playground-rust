# complexity will be O(N^2)

x = [3, 4, 2]
R = []

# rolling sum
s = 0
for i, v in enumerate(x):
    s += v
    R.append(s)

def get_sum_between(i, j):
    """
    The last argument, `j`, is inclusive
    """
    if j >= len(R) or i >= len(R):
        return 0
    if i < 0 or j < 0:
        return 0
    if j < i:
        raise Exception("TODO")
    if i == 0:
        return R[j]
    return R[j]-R[i-1]

for i, _ in enumerate(x):
    for j, _ in enumerate(x):
        if j < i or j == 0:
            continue
        subarray_avg = get_sum_between(i, j)/(j-i+1)
        other_sum_0 = get_sum_between(0, i-1)
        other_sum_1 = get_sum_between(j+1, len(x)-1)
        if j+1 >= len(x):
            if i == 0:
                other_avg = 0
            else:
                other_avg = other_sum_0/i
        else:
            other_avg = (other_sum_0+other_sum_1)/(i+ (j-i+1))

        if subarray_avg > other_avg:
            print(i+1, j+1)
