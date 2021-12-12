import math

COLORS = [
    "#BF1C4C",
    "#7E3BA0",
    "#6D72C3",
    "#8F1E74",
    "#D794BB",
    "#0086A3",
    "#78B0E3",
    "#F4D085",
    "#FFAE7C",
    "#FF778E",
    "#83CCBD"
]

def render(canvas, board):
    padding = 1
    width = canvas.width
    height = canvas.height

    tile_size = min(
        math.floor((width - 4 * padding) / board.width),
        math.floor((height - 4 * padding) / board.height)
    )
    offset_left = round((width - board.width * tile_size) / 2)
    offset_top = round((height - board.height * tile_size) / 2)

    canvas.create_rectangle((0, 0, width, height), fill="#ffffff", outline="")

    # Background for the board
    canvas.create_rectangle((
            offset_left - 3 * padding, offset_top - 3 * padding,
            offset_left + board.width * tile_size + 3 * padding,
            offset_top + board.height * tile_size + 3 * padding
        ),
        fill="#291C34", outline=""
    )

    # Dash out the exit
    canvas.create_line((
            offset_left + board.width * tile_size + padding,
            offset_top + board.exit_y * tile_size + padding,
            offset_left + board.width * tile_size + padding,
            offset_top + (board.exit_y + 1) * tile_size - padding,
        ),
        fill="#ffffff", dash=[4 * padding, 4 * padding], width = 4 * padding
    )

    # Fills in one square of the board
    def rect(x, y, fill):
        canvas.create_rectangle((
                offset_left + x * tile_size + padding,
                offset_top + y * tile_size + padding,
                offset_left + (x + 1) * tile_size - padding,
                offset_top + (y + 1) * tile_size - padding
            ),
            fill=fill, outline=""
        )

    for y in range(0, board.height):
        for x in range(0, board.width):
            fill = "#ffffff"
            car = board.get_car(x, y)
            if car != None:
                fill = COLORS[car % len(COLORS)]
            rect(x, y, fill)
