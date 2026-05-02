"""
File implementing basic matrix operations:
multiplication, determinants, inverse, etc
addition needs to be implented along with some other ones

TODO
    - Addition, subtraction
    - Augmented matrices
    - Symbolic calculations
    - reducing floating point error
    - clean up code cracka
"""

from Solver import vec
import copy
def size(A):
    m=len(A)
    try:
        n=len(A[0])
    except:
        n=1
    return [m,n]

#matrix multiplication
def mult(a,b):
    if len(a[0])!=len(b):
        raise Exception("matrices dont have right dimensions")
    try:
        len_b0=len(b[0])
    except:
        len_b0=1
    
    #rotates matrix b
    b_temp=[]
    for i in range(len_b0):
        k=[]
        for j in range(len(b)):
            k.append(b[j][len_b0-1-i])
        
        b_temp.append(k)
    b=b_temp
    
    c=[]
    for n in range(len(a)):
        k=[]
        for m in range(len(b)):
            k.append(vec.dot_prod(a[n],b[len(b)-1-m]))   
        c.append(k)
    return c

#transpose a matrix
def trans(a):
    len_a=len(a)
    try:
        len_a0=len(a[0])
    except:
        len_a0=1
    
    a_temp=[]
    for n in range(len_a0):
        k=[]
        for m in range(len_a):
            k.append(0)
        a_temp.append(k)
    for n in range(len_a0):
        for m in range(len_a):
            a_temp[n][m]=a[m][n]
    
    return a_temp

#submatrix
def sub_mat(A,m,n):
    A.pop(m)
    for i in range(len(A)):
        A[i].pop(n)
    return A

#determinant
def det(A):
    s=size(A)
    if s[0]!=s[1]:
        raise ValueError("Input must be a square matrix")
    if s[0]==2:
        #determinent of 2 by 2 matrix
        d=A[0][0]*A[1][1]-A[0][1]*A[1][0]
    else:
        d=0
        #recursively calculates determinent using cofactor
        for j in range(s[1]):
            A_sub=sub_mat(copy.deepcopy(A),0,j)       
            cofac = (-1)**j * det(A_sub)
            d+=cofac*A[0][j]
    return d
#adjoint
def adj(A):
    s=size(A)
    if s[0]!=s[1]:
        raise ValueError("Input must be a square matrix")
    
    
    B=[]
    """
    Note: this can be made more efficent by swapping the i and j arguments for the sub_mat call and then removing the transposition entirely
    I have not done this because I want this algorithm to be mathematically representitive  
    """
    for i in range(s[0]):
        E=[]
        for j in range(s[1]):
            A_sub=sub_mat(copy.deepcopy(A),i,j)       
            cofac = (-1)**(i+j) * det(A_sub)
            E.append(cofac)
        B.append(E)
    B=trans(B)
    return B

#multipy matrix by a constant
def mult_const(A,k):
    s=size(A)
    m=s[0]
    n=s[1]
    B=[]
    for i in range(m):
        E=[]
        for j in range(n):
            E.append(k*A[i][j])
        B.append(E)
    return(B)
#inverse of matrix
def inv(A):
    k=1/det(A)
    B=adj(A)
    B=mult_const(B,k)
    return B