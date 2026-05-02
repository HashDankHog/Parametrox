import mat
import func 
import math
a=[[1,1,1],
   [-1,3,2],
   [2,1,1]]
b=[[5],
   [2],
   [1]]
def f(V):
    return math.sin(V[0])+math.cos(V[1])

print("\n", func.grad_des(f,[0.5,0.5]))
print(math.cos(0.5))
