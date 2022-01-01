from board import Board

def parse(string):
    board = Board(6, 6)
    cars = dict() # Cars are stored as a tuple (x, y, length, direction)

    table = [[-1 for x in range(board.width)] for y in range(board.height)]

    x = 0
    y = 0
    for char in string:
        if char == "\n":
            y += 1
            x = 0
        else:
            char = ord(char)
            if char >= 48 and char < 58: # [0-9]
                table[y][x] = char - 48
            elif char >= 97 and char < 123: # [a-z]
                table[y][x] = char - 97 + 10
            elif char >= 65 and char < 91: # [A-Z]
                table[y][x] = char - 65 + 10
            x += 1

    for y in range(board.height):
        for x in range(board.width):
            if table[y][x] >= 0:
                if table[y][x] not in cars:
                    length = 0
                    # See if the car is horizontal or vertical by trying both directions
                    if x < board.width - 1 and table[y][x + 1] == table[y][x]:
                        for x2 in range(x, board.width):
                            if table[y][x2] == table[y][x]:
                                length += 1
                            else:
                                break
                        cars[table[y][x]] = (x, y, length, True)
                    elif y < board.height - 1:
                        for y2 in range(y, board.height):
                            if table[y2][x] == table[y][x]:
                                length += 1
                            else:
                                break
                        cars[table[y][x]] = (x, y, length, False)

    for key in sorted(cars.keys()):
        board.cars.append(cars[key])
    board.exit_y = cars[0][1]

    return board
