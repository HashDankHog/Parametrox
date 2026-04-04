class gate:
    """
    this list of fields needs to be converted to just
        x
        y
        type
        tag
        input1
        input2
    everything else can be internally computed
    """
    def __init__(self,x0,y0,x1,y1,x2,y2,x3,y3,t,tag,input1,input2):
        self.x_center=x0
        self.y_center=y0

        self.x_input1=x1
        self.y_input1=y1

        self.x_input2=x2
        self.y_input2=y2
        
        self.x_output=x3
        self.y_output=y3
        
        self.gate_type=t
        
        self.input1=input1
        self.input2=input2
        self.output=False
        self.gate_tag=tag
        
        self.total_width=140
        self.total_height=50
    def update(self):
        try:
            input1=self.input1.output
        except:
            input1=False
        try:
            input2=self.input2.output
        except:
            input2=False
            
        if self.gate_type=="nand":
            self.output=not(input1 and input2)
        elif self.gate_type=="not":
            self.output=not input1
        elif self.gate_type=="and":
            self.output=input1 and input2
        elif self.gate_type=="or":
            self.output=input1 or input2
        elif self.gate_type=="nor":
            self.output=not(input1 or input2)
        elif self.gate_type=="xor":
            self.output=input1 ^ input2
        elif self.gate_type=="input":
            self.output = not self.output
        elif self.gate_type=="output":
            self.output= self.input1


gate_list = []