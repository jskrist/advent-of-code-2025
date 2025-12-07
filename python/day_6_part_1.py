import numpy as np
from core import read_file, parse_inputs


def solution(scenario: str):
    input_file = scenario + ".txt"
    input = read_file(input_file)

    lines = np.array([line.strip().split() for line in input.strip().splitlines()])
    numerical_data = np.array([[int(d) for d in line] for line in lines
                               if line[0] not in ["+", "*"]]).T

    result = 0
    for op, nums in zip(lines[-1], numerical_data):
        if op == "*":
            result += nums.prod()
        elif op == "+":
            result += nums.sum()

    print(f"grand total: {result}\n")


if __name__ == "__main__":
    scenario = parse_inputs("day_6_test_1")
    solution(scenario)
