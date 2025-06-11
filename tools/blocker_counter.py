BOARD_SIZE = 8

def on_board(x, y):
    return 0 <= x < BOARD_SIZE and 0 <= y < BOARD_SIZE

def generate_relevant_squares(x: int, y: int, directions: list[tuple[int, int]]) -> list[tuple[int, int]]:
    relevant = []

    for dx, dy in directions:
        nx, ny = x + dx, y + dy

        while on_board(nx, ny):
            # Skip edge squares
            if not (nx == 0 or nx == 7 or ny == 0 or ny == 7):
                relevant.append((nx, ny))
            else:
                break

            nx += dx
            ny += dy

    return relevant

def square_name(x: int, y: int):
    return chr(ord('a') + x) + str(y + 1)

def find_max_blocker_configurations(directions: list[tuple[int, int]]):
    max_relevant = 0
    max_square = (-1, -1)

    for y in range(8):
        for x in range(8):
            relevant = generate_relevant_squares(x, y, directions)
            count = len(relevant)
            if count > max_relevant:
                max_relevant = count
                max_square = (x, y)

    square = square_name(max_square[0], max_square[1])
    print(f"Max relevant squares: {max_relevant} at {square}")
    print(f"Max blocker permutations: 2^{max_relevant} = {2 ** max_relevant}")

dirs = [
    (1, 0),   # → East
    (-1, 0),  # ← West
    (0, 1),   # ↑ North
    (0, -1),  # ↓ South

    (1, 1),   # ↗ NE
    (-1, 1),  # ↖ NW
    (-1, -1), # ↙ SW
    (1, -1),  # ↘ SE
]
find_max_blocker_configurations(dirs)
