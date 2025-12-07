from itertools import groupby
import math
import numpy as np
from core import read_file, parse_inputs


def split_list_by_value(lst, value):
    return [list(group) for key, group in groupby(lst, lambda x: x == value) if not key]


def solution(scenario: str):
    input_file = scenario + ".txt"
    input = read_file(input_file)

    all_chars = list(input)
    lines = np.array(split_list_by_value(all_chars, "\n"))
    num_strs = ["".join(line).strip() for line in lines[:-1].T.tolist()]
    prob_groups = [[int(d) for d in line] for line in split_list_by_value(num_strs, "")]

    result = 0
    for op, nums in zip(np.delete(lines[-1], np.where(lines[-1] == " ")), prob_groups):
        if op == "*":
            result += math.prod(nums)
        elif op == "+":
            result += sum(nums)

    print(f"grand total: {result}\n")


if __name__ == "__main__":
    scenario = parse_inputs("day_6_test_1")
    solution(scenario)
