def main():
    with open("./input_long.txt") as file:
        input = file.read()
    score = solve_part1(input)
    print(score)


def solve_part1(text: str) -> int:
    # Map moves to numbers
    move_value = {
        "A": 0,  # Rock
        "B": 1,  # Paper
        "C": 2,  # Scissors
        "X": 0,  # Rock
        "Y": 1,  # Paper
        "Z": 2,  # Scissors
    }

    total_score = 0

    for line in text.splitlines():
        if not line:
            continue

        opponent_code, my_code = line.split()

        opponent_move = move_value[opponent_code]
        my_move = move_value[my_code]

        # Shape score: Rock=1, Paper=2, Scissors=3
        shape_score = my_move + 1

        # Determine outcome
        result = (my_move - opponent_move) % 3

        if result == 0:
            outcome_score = 3  # draw
        elif result == 1:
            outcome_score = 6  # win
        else:
            outcome_score = 0  # loss

        total_score += shape_score + outcome_score

    return total_score


if __name__ == "__main__":
    main()
