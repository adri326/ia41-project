import sys
import tkinter as tk
import threading
from os.path import exists

from render import render
from board import Board
from canvas import ResizingCanvas
from solve import bfs
from parse import parse

root = tk.Tk()
root.title("IA41 Project: Rush Hour")

main_frame = tk.Frame(root, padx=10, pady=10, bg="white")
main_frame.grid(column=0, row=0, sticky="nwes")
main_frame.pack(fill="both", expand=True)

canvas = ResizingCanvas(main_frame, bg="white")
canvas.grid(column=0, row=0, sticky="nwes")
canvas.pack(fill="both", expand=True)

board = None

if len(sys.argv) > 1:
    if exists(sys.argv[1]):
        f = open(sys.argv[1], 'r', encoding='utf8', errors='ignore')
        string = f.read()
        f.close()
        board = parse(string)

if board == None:
    board = Board(6, 6)
    board.cars.append((3, 2, 2, True)) # x, y, length, horizontal
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
