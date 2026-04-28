"""
file implementing useful operations on functions:
Linearization, integration, root finding, differentiation, etc

TODO:
    - partial differtiation/integration
    - rewrite linearization function for fractions and for augmented matrices
    - add support for parametrics
    - improve root finding
    - gradient descent 
    - intersections
        - ?
"""
import math

#approximates f'(x)
def diff(f,x,dx=0.05):
    x0=x
    y0=f(x0)

    x1=x+dx
    y1=f(x1)

    try:
        m=(y1-y0)/(x1-x0)
    except ZeroDivisionError:
        m=math.inf
    
    return m

# approximates definite integral of f(x) from x0 to x1
def inte(f,x0,x1,step=2000):
    dx=(x1-x0)/step
    x=x0
    
    y=f(x)

    a=0
    for i in range(step):  
        a+=y*dx
        x+=dx
        y=f(x)
    return a

# linearizes f at x0
def lin(f,x0,dx=0.05):
    y0=f(x0)
    m=diff(f,x0,dx=dx)
    
    if m==math.inf:
        a=1
        b=0
        c=x0
    else:
        a=-m
        b=1
        c=y0-m*x0
    return [a,b], c

#newton-raphsom root finding
def root(f,X0,max_iter=20,dx=0.05,res=6):
    iter=0
    x=X0
    while not(round(f(x),res)==0 or iter>=max_iter):
        [a,b], c = lin(f,x,dx=dx)
        x=c/a
        iter+=1
    return round(x,res)

"""
Optimal Practices:
    - x_min should be barely less than the expected minimum root and the converse for x_max
    - seg should be the suspected number of roots minus 1
"""
#finds all of the roots for a range
def root_ran(f,x_min,x_max,seg=1,res=6,dx=0.05,max_iter=20):
    #default number of segments is one because most root finding problems will be quadratic in a CAD setting
    step=(x_max-x_min)/seg
    x=x_min
    roots=[]
    for i in range(seg+1):
        roots.append(root(f,x,max_iter=max_iter,dx=dx,res=res))
        x+=step
    return list(set(roots))

