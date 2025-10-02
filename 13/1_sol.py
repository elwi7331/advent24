import numpy as np
from hsnf import smith_normal_form
import re

def parse_input(path):
    with open(path) as f:
        total_cost = 0
        for line in f:
                a_x_match = re.search(r"Button A: X\+(.+?), ", line)
                a_y_match = re.search(r", Y\+(.+?)\n", line)
                b_x_match = re.search(r"Button B: X\+(.+?), ", line)
                b_y_match = re.search(r", Y\+(.+?)\n", line)
                
                target_x_match = re.search(r"Prize: X=(.+?),", line)
                target_y_match = re.search(r", Y=(.+?)\n", line)
                
                if a_x_match and a_y_match:
                    a_x = int(a_x_match.group(1))
                    a_y = int(a_y_match.group(1))
                elif b_x_match and b_y_match:
                    b_x = int(b_x_match.group(1))
                    b_y = int(b_y_match.group(1))
                elif target_x_match and target_y_match:
                    target_x = int(target_x_match.group(1))
                    target_y = int(target_y_match.group(1))
                else:
                    A = np.array([
                        [a_x, b_x],
                        [a_y, b_y]
                    ])
                    C = np.array([target_x, target_y])
                    ans = solve(A, C)
                    if ans is not None:
                        X = np.dot(ans[1], ans[0])
                        if X[0] <= 100 and X[1] <= 100:
                            Z = np.array([3, 1]) # cost per button
                            total_cost += np.dot(X, Z)
    return total_cost

def solve(A, C):
    m, n = (A.shape[0], A.shape[1])
    
    B, U, V = smith_normal_form(A)

    # k becomes the number of diagonal non zeros
    k = 0
    while k < m and B[k][k] != 0:
        k += 1

    D = np.dot(U, C)
    Y = np.zeros(n)
    
    for i in range(0, k):
        if D[i] % B[i][i] == 0:
            Y[i] = D[i] / B[i][i]
        else:
            return None
    for i in range(k, n):
        if D[i] != 0:
            return None
    
    return (Y, V)

 
if __name__ == '__main__':
    cost = parse_input("./input.txt")
    print(cost)
