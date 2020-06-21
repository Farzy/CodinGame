import sys
import math

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.
# ---
# Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.

# light_x: the X position of the light of power
# light_y: the Y position of the light of power
# initial_tx: Thor's starting X position
# initial_ty: Thor's starting Y position
light_x, light_y, initial_tx, initial_ty = [int(i) for i in input().split()]

move_x = {
    -1: "W",
    0: "",
    1: "E"
}

move_y = {
    -1: "N",
    0: "",
    1: "S"
}

# game loop
while True:
    remaining_turns = int(input())  # The remaining amount of turns Thor can move. Do not remove this line.

    # Write an action using print
    # To debug: print("Debug messages...", file=sys.stderr, flush=True)
    dx = light_x - initial_tx
    if dx < 0:
        dx = -1
    elif dx > 0:
        dx = 1
    dy = light_y - initial_ty
    if dy < 0:
        dy = -1
    elif dy > 0:
        dy = 1
    if dx or dy:
        initial_tx += dx
        initial_ty += dy
        # A single line providing the move to be made: N NE E SE S SW W or NW
        print(move_y[dy] + move_x[dx])
