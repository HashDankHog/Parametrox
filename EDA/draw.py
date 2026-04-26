from tkinter import *
import canvas as canvas
import gates as gates
def nand(x,y):
    main_width=100
    total_height=50
    arc_size=50
    not_size=10
    line_length=20
    line_distance=20
    width_offset=(main_width-arc_size)/2
    height_offset=total_height/2
    line_offset=line_distance/2
    
    x1=x-width_offset-line_length
    y1=y+line_offset
    y2=y-line_offset
    x3=x+width_offset+arc_size/2+not_size+line_length
    tag_body=str(len(gates.gate_list))+"nand"
    gate = gates.gate(x,y,x1,y1,x1,y2,x3,y,"nand",tag_body,[],[])
    gates.gate_list.append(gate)

    canvas.screen.create_line(x-width_offset,y+height_offset,x+width_offset,y+height_offset,fill="blue",tags=tag_body)
    canvas.screen.create_line(x-width_offset,y-height_offset,x+width_offset,y-height_offset,fill="blue",tags=tag_body)
    canvas.screen.create_line(x-width_offset,y-height_offset,x-width_offset,y+height_offset,fill="blue",tags=tag_body)
    canvas.screen.create_arc(x+width_offset-arc_size/2,y+height_offset,x+width_offset+arc_size/2,
                             y-height_offset,start=90,extent=-180,outline="blue",style="arc",tags=tag_body)
    
    canvas.screen.create_oval(x+width_offset+arc_size/2,y-not_size/2,
                              x+width_offset+arc_size/2+not_size,y+not_size/2,outline="blue",tags=tag_body)
    
    canvas.screen.create_line(x-width_offset-line_length,y+line_offset,x-width_offset,y+line_offset,fill="blue",tag=tag_body)
    canvas.screen.create_line(x-width_offset-line_length,y-line_offset,x-width_offset,y-line_offset,fill="blue",tag=tag_body)
    canvas.screen.create_line(x+width_offset+arc_size/2+not_size,y,
                              x+width_offset+arc_size/2+not_size+line_length,y,fill="blue",tag=tag_body)
def input(**kwargs):
    input_radius=20
    line_length=20
    args_length=len(kwargs.items())
    if args_length==1:
        output=kwargs["gate"].output 
        x=kwargs["gate"].x_center
        y=kwargs["gate"].y_center
        tag=kwargs["gate"].gate_tag
    else:
        output=False
        x=kwargs["x"]
        y=kwargs["y"]
        tag=str(len(gates.gate_list))+"input"
        gate = gates.gate(x,y,-50,-50,-50,-50,x,y+input_radius+line_length,"input",tag,0,0)
        gates.gate_list.append(gate)



    canvas.screen.create_line(x,y+input_radius,x,y+input_radius+line_length,fill="blue",tag=tag)
    if output:
        canvas.screen.create_rectangle(x-input_radius,y-input_radius,x+input_radius,y+input_radius,
                                       fill="red",outline="blue", tag=tag)
    else:
        canvas.screen.create_rectangle(x-input_radius,y-input_radius,x+input_radius,y+input_radius,
                                       outline="blue",fill="white",tag=tag)


def output(**kwargs):
    output_radius=20
    line_length=20
    args_length=len(kwargs.items())
    if args_length==1:
        input=kwargs["gate"].input1.output
        x=kwargs["gate"].x_center
        y=kwargs["gate"].y_center
        tag=kwargs["gate"].gate_tag
    else:
        input=False
        x=kwargs["x"]
        y=kwargs["y"]
        tag=str(len(gates.gate_list))+"input"
        gate = gates.gate(x,y,x,y+output_radius+line_length,-50,-50,-50,-50,"output",tag,0,0)
        gates.gate_list.append(gate)


    
    canvas.screen.create_line(x,y+output_radius,x,y+output_radius+line_length,fill="blue",tag=tag)
    if input:
        canvas.screen.create_oval(x-output_radius,y-output_radius,x+output_radius,y+output_radius,
                                       fill="red",outline="blue", tag=tag)
    else:
        canvas.screen.create_oval(x-output_radius,y-output_radius,x+output_radius,y+output_radius,
                                       outline="blue", fill="white",tag=tag)
        



def path(gate_1,gate_2,gate_2_input):
    if gate_2_input==1:
        canvas.screen.create_line(gate_1.x_output,gate_1.y_output,gate_2.x_input1,gate_2.y_input1,tag=gate_1.gate_tag,fill="red")
    else:
        canvas.screen.create_line(gate_1.x_output,gate_1.y_output,gate_2.x_input2,gate_2.y_input2,tag=gate_1.gate_tag,fill="red")
    
def path_not_use(x1,y1,x2,y2):
    """
    this segment just creates a list of every visitable point for any potential lines
    there are multiple GIANT improvements that should be made down the line
        - the bitmap doesnt have to be entirely computed every time, only the changes do
        - some of the logic can be simplified with xor's
        - this could be wildely improved with better data structures
    in short, this isnt efficient, at all
    for now, im just going to not use it
    """
    width=canvas.width    
    height=canvas.height
    allowed_positions=[]
    for y in range(width):
        for x in range(height):
            allowed_positions.append([x,y])
    not_allowed_positions=[]
    for gate in gates.gate_list:
        for y in range(gate.total_height):
            for x in range(gate.total_width):
                x_adjusted=gate.center_x+x-gate.total_width/2
                y_adjusted=gate.center_y+y-gate.total_height/2
                not_allowed_positions.append(x_adjusted,y_adjusted)
    
    allowed_positions=[position for position in allowed_positions if position not in not_allowed_positions]
  
    