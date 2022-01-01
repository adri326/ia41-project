import sys
import tkinter as tk
import threading
from render import render
from board import Board
from canvas import ResizingCanvas
from solve import bfs

root = tk.Tk()
root.title("IA41 Project: Rush Hour")

main_frame = tk.Frame(root, padx=10, pady=10, bg="white")
main_frame.grid(column=0, row=0, sticky="nwes")
main_frame.pack(fill="both", expand=True)

canvas = ResizingCanvas(main_frame, bg="white")
canvas.grid(column=0, row=0, sticky="nwes")
canvas.pack(fill="both", expand=True)

board = Board(6, 6)

try:
	f = open("example_board",'r', encoding='utf8', errors='ignore') 

	cars_file = [[-1,-1,0,1],[-1,-1,0,1],[-1,-1,0,1],[-1,-1,0,1],[-1,-1,0,1],[-1,-1,0,1],[-1,-1,0,1],[-1,-1,0,1],[-1,-1,0,1],[-1,-1,0,1],[-1,-1,0,1],[-1,-1,0,1]]

	c = [[0,0,0,0,0,0],[0,0,0,0,0,0],[0,0,0,0,0,0],[0,0,0,0,0,0],[0,0,0,0,0,0],[0,0,0,0,0,0]]

	# note: after digits from 0 to 9, the ASCII characters are :;<=>

	for i in range (6):
		for j in range (6) :
			c[i][j] = ord(f.read(1))-48
			print(c[i][j])
		f.read(1)
		
	f.close()

	for i in range (12):
		for j in range (6) :
			for k in range (6) :
				if	c[j][k] == i+1 and cars_file[i][0] == -1 :
					cars_file[i][0] = k
					cars_file[i][1] = j
					cars_file[i][2] += 1
				elif c[j][k] == i+1 and cars_file[i][1] != j :
					cars_file[i][2] += 1
					cars_file[i][3] = 0
				elif c[j][k] == i+1:
					cars_file[i][2] += 1
				

	for l in range (12):
		if cars_file[l][0] != -1:
			board.cars.append((cars_file[l][0],cars_file[l][1],cars_file[l][2],cars_file[l][3]))

except BaseException:
	board.cars.append((3, 2, 2, True)) # x, y, length, horizontal?
	board.cars.append((0, 0, 2, True))
	board.cars.append((2, 0, 2, True))
	board.cars.append((5, 0, 2, False))
	board.cars.append((0, 1, 2, False))
	board.cars.append((1, 1, 2, True))
	board.cars.append((2, 2, 2, False))
	board.cars.append((5, 2, 2, False))
	board.cars.append((0, 3, 2, True))
	board.cars.append((3, 3, 2, False))
	board.cars.append((2, 4, 2, False))
	board.cars.append((4, 4, 2, True))

canvas.on_resize = lambda _: render(canvas, board)

render(canvas, board)
main_frame.update()

def solve():
    solution = bfs(board)

    def timer(secs):
        def fn():
            global board
            global canvas
            if len(solution) > 0:
                board = board.move(*solution.pop(0))
                # Bad, but the documentation on how to do this better is ~gone~
                render(canvas, board)
                timer(secs)
        threading.Timer(secs, fn).start()

    timer(1)

threading.Timer(0, solve).start()

root.mainloop() # TODO: remove
