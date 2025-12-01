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
offsets = np.array([int(i) for i in num_strs.splitlines()])
password = 0
current_value = starting_point
for offset in offsets:
    next_value = current_value + offset
    # Check for zero
    if next_value == 0:
        password += 1
    elif current_value > 0 and next_value < 0:
        password += 1
    # Check for wrap around
    password += abs(next_value) // 100

    current_value = np.mod(next_value, 100)

# vectorized approach double counts some crossings, not sure why
# # calculate the running total
# running_total = np.cumsum(np.insert(offsets, 0, starting_point))
# real_vals = np.mod(running_total, 100)
# next_values = real_vals[1:] + offsets
# zero_count = sum(real_vals == 0)
# wrap_count = sum(abs(offsets) // 100)
# negative_crossing_count = sum((real_vals[:-1] > 0) & (next_values < 0))
# password = zero_count + wrap_count + negative_crossing_count

# print how many times the value 0 is reached
print(f"The password is {password}")
