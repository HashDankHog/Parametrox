#just a couple of very basic vector operationss
#most of the more complex algorithms and operations will be part of the matrix file(s)
def dot_prod(a,b):
    c=0
    len_a=len(a)
    len_b=len(b)
    try:
        len_a0=len(a[0])
    except:
        len_a0=1
    try:
        len_b0=len(b[0])
    except:
        len_b0=1
    
    if len_a != len_b:
        raise Exception("Vectors must be of same length in order to compute a dot product")
    elif len_a0!=1 or len_b0!=1:
        raise ValueError("Inputs must be vectors")
    for i in range(len_a):
        c+= a[i]*b[i]
    return c
def cross_prod(a,b):
    len_a=len(a)
    len_b=len(b)
    try:
        len_a0=len(a[0])
    except:
        len_a0=1
    try:
        len_b0=len(b[0])
    except:
        len_b0=1
    
    if len_a!= len_b:
        raise Exception("Vectors must be of same length in order to compute a dot product")
    elif len_a0!=1 or len_b0!=1:
        raise ValueError("Inputs must be vectors")
    elif len_a == 7:
        raise ValueError("son im crine nobody needs a seven dimensional cross product")
    elif len_a != 3:
        raise ValueError("Cross products must be on vectors of length 3")
    
    c=[]
    c.append(a[1]*b[2]-a[2]*b[1])
    c.append(a[2]*b[0]-a[0]*b[2])
    c.append(a[0]*b[1]-a[1]*b[0])
   
    return c

def add(a,b):
    len_a=len(a)
    len_b=len(b)
    try:
        len_a0=len(a[0])
    except:
        len_a0=1
    try:
        len_b0=len(b[0])
    except:
        len_b0=1

    if len_a!= len_b:
        raise Exception("Vectors must be of same length in order to compute a dot product")
    elif len_a0!=1 or len_b0!=1:
        raise ValueError("Inputs must be vectors")
    c=[]
    for i in range(len_a):
        c.append(a[i]+b[i])
    return c
def sub(a,b):
    len_a=len(a)
    len_b=len(b)
    try:
        len_a0=len(a[0])
    except:
        len_a0=1
    try:
        len_b0=len(b[0])
    except:
        len_b0=1
    
    if len_a!= len_b:
        raise Exception("Vectors must be of same length in order to compute a dot product")
    elif len_a0!=1 or len_b0!=1:
        raise ValueError("Inputs must be vectors")
    c=[]
    for i in range(len_a):
        c.append(a[i]-b[i])
    return c

def mult(a,k):
    try:
        if len(a[0])!=1:
            raise ValueError("Input a must be a matrix")
        elif not(type(k) == int or type(k) == float):
            raise ValueError("Input k must be a integer or a float")
    except:
        pass
    c=[]
    for i in range(len(a)):
        c.append(a[i]*k)
    return c
