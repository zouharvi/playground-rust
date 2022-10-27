from typing import List
import math

def rotate(matrix: List[List[int]]) -> None:
    """
    Do not return anything, modify matrix in-place instead.
    """
    n = len(matrix)
    n_effective_u = math.ceil(n/2)
    n_effective_d = math.floor(n/2)
    # columns
    for a_i in range(n_effective_u):
        # rows
        for a_j in range(n_effective_d):
            a = matrix[a_j][a_i]
            # a will go here
            b_i = -a_j-1
            b_j = a_i
            b = matrix[b_j][b_i]
            # b will go here
            c_i = -b_j-1
            c_j = b_i
            c = matrix[c_j][c_i]
            # c will go here
            d_i = -c_j-1
            d_j = c_i
            d = matrix[d_j][d_i]

            # set b to a
            matrix[b_j][b_i] = a
            # set c to b
            matrix[c_j][c_i] = b
            # set d to c
            matrix[d_j][d_i] = c
            # set a to d
            matrix[a_j][a_i] = d

            # print("a:", a, " - ", a_i, a_j)
            # print("b:", b, " - ", b_i, b_j)
            # print("c:", c, " - ", c_i, c_j)
            # print("d:", d, " - ", d_i, d_j)

    # return matrix


# print(rotate([[1,2],[3,4]]))
print(rotate([[1,2,3],[4,5,6],[7,8,9]]))
print(rotate([[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]))
