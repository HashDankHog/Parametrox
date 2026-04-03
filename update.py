import draw
import gates
import canvas
from math import *
event_type="nand"
path_input=0
path_output=0
input=1
def set_nand():
    global event_type
    event_type = "nand"
def set_output():
    global event_type
    event_type = "output"
def set_input():
    global event_type
    event_type = "input"
def flip():
    global event_type
    if event_type =="nand":
        event_type="path"
    else:
        event_type="nand"
def place(event):
    x=event.x
    y=event.y
    global event_type
    global path_input
    global path_output
    global input
    if event_type=="path":
        #WHY DOES THIS WORK I FEEL LIKE SIMULTIOUSLY A GOD AT CODING AND THE WORLDS WORST CODER WTF
        #WHY HAS ALL OF THE CODE FOR THIS PROJECT REQUIRED NO DEBUGGING(yet)
        #I HAVE 0 CLUE HOW THIS WORKS, AGAIN
        user_error=10
        for gate in gates.gate_list:
            if dist([gate.x_input1,gate.y_input1],[x,y])<=user_error:
                input=1
                path_input=gate
            elif dist([gate.x_input2,gate.y_input2],[x,y])<=user_error:
                input=2
                path_input=gate
            elif dist([gate.x_output,gate.y_output],[x,y])<=user_error:
                path_output=gate
            if gate.gate_type=="input" and dist([gate.x_center,gate.y_center],[x,y])<30:
                gate.update()
                draw.input(gate=gate)
            
        if path_input!=0 and path_output!=0:
            if input==1:
                path_input.input1=path_output
            else:
                path_input.input2=path_output
            
            draw.path(path_output,path_input,input)
            path_input=0
            path_output=0
        
    
    else:
        for gate in gates.gate_list:
            min_x=gate.x_center-gate.total_width
            max_x=gate.x_center+gate.total_width
            
            min_y=gate.y_center-gate.total_height
            max_y=gate.y_center+gate.total_height
            if x > min_x and x < max_x:
                if y > min_y and y < max_y:
                    return 0
        
        if event_type=="nand":
            draw.nand(x,y)
        elif event_type=="input":
            draw.input(x=x,y=y)
        elif event_type=="output":
            draw.output(x=x,y=y)
    event_type="path"

def delete(event):
    x=event.x
    y=event.y
    global gate_type
    n=0
    for gate in gates.gate_list:
        min_x=gate.x_center-gate.total_width
        max_x=gate.x_center+gate.total_width
        
        min_y=gate.y_center-gate.total_height
        max_y=gate.y_center+gate.total_height
        if x > min_x and x < max_x:
            if y > min_y and y < max_y:
                tag_body=gate.gate_tag
                gates.gate_list.pop(n)
                canvas.screen.delete(tag_body)
        n+=1
def update():
    canvas.screen
    for gate in gates.gate_list:
        if gate.gate_type == "input":
            pass
        else:
            gate.update()
        if gate.gate_type == "output":
            print(gate.input1.output)
            draw.output(gate=gate)
