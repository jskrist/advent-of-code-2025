from copy import copy
import re
from core import read_file, parse_inputs


def solution(scenario: str):
    input_file = scenario + ".txt"
    input = read_file(input_file)
    start_stops = re.findall(r'(\d+)-(\d+)', input)
    start_stops = [[int(start_stop[0]), int(start_stop[1])] for start_stop in start_stops]

    to_check = copy(start_stops)
    combined_ranges = [copy(start_stops[0])]
    while to_check:
        broke = False
        for idx, next_range in enumerate(to_check):
            if (next_range[0] <= combined_ranges[-1][1]) & \
               (next_range[1] >= combined_ranges[-1][0]):
                combined_ranges[-1][0] = min(combined_ranges[-1][0], next_range[0])
                combined_ranges[-1][1] = max(combined_ranges[-1][1], next_range[1])
                del to_check[idx]
                broke = True
                break
        if not broke and to_check:
            combined_ranges.append(to_check.pop())

    num_fresh_ids = 0
    for range in combined_ranges:
        num_fresh_ids += range[1] - range[0] + 1

    print(f"number of fresh ids: {num_fresh_ids}\n")


if __name__ == "__main__":
    scenario = parse_inputs("day_5_test_1")
    solution(scenario)
