from tkinter import *
import update as update
import canvas as canvas

canvas.screen.bind("<Button-1>", update.place)
canvas.screen.bind("<Button-2>", update.delete)
menu_screen = Menu(canvas.root)
place = Menu(menu_screen,tearoff=0)
menu_screen.add_cascade(label='Place', menu = place)
place.add_command(label="NAND",command=update.set_nand)
place.add_command(label="Output",command=update.set_output)
place.add_command(label="Input",command=update.set_input)

menu_screen.add_command(label="run",command=update.run)
menu_screen.add_command(label="stop",command=update.stop)

canvas.root.config(menu=menu_screen)
canvas.root.mainloop()