import numpy as np
import time

start = time.time()

coeff = [3.2, 2, 1]
f = open("roots.txt", "w")
for i in range(1, (2**25)):
    coeff = []
    while i > 0:
        coeff.insert(0, ((i % 2) * 2) - 1)
        i = i // 2
    for root in np.roots(coeff):
        f.write(str(root) + "\n")
    negcoeff = [j * -1 for j in coeff]
    for root in np.roots(negcoeff):
        f.write(str(root) + "\n")
end = time.time()
print(end - start)
