from core import read_file, parse_inputs

num_digits = 12


def solution(scenario: str):
    input_file = scenario + ".txt"
    input = read_file(input_file)

    total_joltage = 0
    for line in input.splitlines():
        max_line_joltage = 0
        start_ind = 0
        for idx in range(num_digits):
            stop_ind = len(line) - num_digits + idx + 1
            subset = [d for d in line[start_ind:stop_ind]]
            max_digit = max(subset)
            start_ind += subset.index(max_digit) + 1
            max_line_joltage *= 10
            max_line_joltage += int(max_digit)
        total_joltage += max_line_joltage
    print(f"Total Joltage: {total_joltage}")


if __name__ == "__main__":
    scenario = parse_inputs("day_3_test_1")
    solution(scenario)
