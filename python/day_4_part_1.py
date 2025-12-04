import numpy as np
from scipy import signal
from core import read_file, parse_inputs


def solution(scenario: str):
    input_file = scenario + ".txt"
    input = read_file(input_file)
    matrix = np.array([[int(c) for c in row]
                      for row in input.replace('.', '0').replace('@', '1').splitlines()])
    kernel = np.array([[1, 1, 1], [1, 0, 1], [1, 1, 1]])
    neighbor_count = signal.convolve2d(matrix, kernel, boundary='fill', mode='same')
    num_accessible = sum(sum((neighbor_count < 4) & (matrix == 1)))
    print(f"number of accessible rolls: {num_accessible}\n")


if __name__ == "__main__":
    scenario = parse_inputs("day_4_test_1")
    solution(scenario)
