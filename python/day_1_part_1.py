from core import read_file, parse_inputs
import numpy as np


def solution(scenario: str):
    # read in the input file
    input_file = scenario + ".txt"
    input = read_file(input_file)
    # define the initial value
    starting_point = 50
    # process the input file (R and L to + and -)
    num_strs = input.replace("L", "-").replace("R", "+")
    # convert to list of integers
    offsets = [int(i) for i in num_strs.splitlines()]
    # calculate the running total
    cum_sum = np.mod(np.cumsum(offsets) + starting_point, 100)

    # determine how many times the value 0 is reached
    password = np.count_nonzero(cum_sum == 0)
    print(f"The password is {password}")


if __name__ == "__main__":
    scenario = parse_inputs("day_1_test_1")
    solution(scenario)
