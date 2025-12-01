import numpy as np

# define the initial value
starting_point = 50
# read in the input file
# input_file = "day_1_puzzle_1_test_1.txt"
input_file = "day_1_puzzle_1_input.txt"
with open(input_file, "r") as file:
    directions = file.read().strip()
# process the input file (R and L to + and -)
num_strs = directions.replace("L", "-").replace("R", "+")
# convert to list of integers
nums = [int(i) for i in num_strs.splitlines()]
# calculate the running total
cum_sum = np.mod(np.cumsum(nums) + starting_point, 100)

# determine how many times the value 0 is reached
zero_crossings = np.sum(cum_sum == 0)
print(f"The password is {zero_crossings}")
