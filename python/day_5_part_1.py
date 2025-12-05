import re
import numpy as np
from core import read_file, parse_inputs


def solution(scenario: str):
    input_file = scenario + ".txt"
    input = read_file(input_file)
    start_stops = re.findall(r'(\d+)-(\d+)', input)
    start_stops = np.array([[int(start_stop[0]), int(start_stop[1])] for start_stop in start_stops])
    ids = np.array([int(x) for x in re.findall(r'(?<=\n)(\d+)(?:\r?\n|$)', input)])
    num_fresh_ids = np.count_nonzero([np.any((id >= start_stops[:, 0]) & (id <= start_stops[:, 1]))
                                      for id in ids])
    print(f"number of fresh ingredients: {num_fresh_ids}\n")


if __name__ == "__main__":
    scenario = parse_inputs("day_5_test_1")
    solution(scenario)
