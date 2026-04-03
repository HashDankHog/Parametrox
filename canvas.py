# Import Module
from tkinter import *

# create root window
root = Tk()

# root window title and dimension
root.title("circuit_sim")
# Set geometry (widthxheight)
root.geometry('1500x1000')
width=1500
height=1000
screen=Canvas(root,width=width,height=height,bg="white")
screen.pack()
# all widgets will be here
# Execute Tkinter
