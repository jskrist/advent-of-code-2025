import numpy as np
import re
from core import read_file, parse_inputs


def solution(scenario: str):
    input_file = scenario + ".txt"
    input = read_file(input_file)
    start_stops = re.findall(r'(\d+)-(\d+)', input)
    start_stop_groups = [[int(start_stop[0]), int(start_stop[1])] for start_stop in start_stops]

    combined_ranges = np.array([], dtype=np.int64)
    for range in start_stop_groups:
        start = range[0]
        stop = range[1]
        comb_len = combined_ranges.size
        if comb_len == 0:
            combined_ranges = np.insert(combined_ranges, 0, range)
            continue
        # find location where this would be inserted
        start_inds = np.where(combined_ranges < start)[0]
        # if there aren't any value before this start, insert it at the start
        if start_inds.size == 0:
            start_ind = np.array(-2)
        else:
            start_ind = start_inds[-1] + 1
        # find the location where the stop would be inserted
        stop_inds = np.where(combined_ranges <= stop)[0]
        if stop_inds.size == 0:
            stop_ind = np.array(-1)
        else:
            stop_ind = stop_inds[-1] + 1
        # if it is outside of a group, we will insert it into the list
        start_in_range = start_ind % 2 == 1 and not start_ind < 0 and not start_ind >= comb_len
        stop_in_range = stop_ind % 2 == 1 and not stop_ind < 0 and not stop_ind >= comb_len

        # clip the values to be inside the array
        start_ind = max(start_ind, 0)
        stop_ind = max(stop_ind, 0)
        # delete any of the elements between the start and stop locations
        combined_ranges = np.delete(combined_ranges, slice(start_ind, stop_ind))
        if not start_in_range:
            combined_ranges = np.insert(combined_ranges, start_ind, start)
            stop_ind = start_ind + 1
        else:
            stop_ind = start_ind

        if not stop_in_range:
            combined_ranges = np.insert(combined_ranges, stop_ind, stop)

    num_fresh_ids = sum(combined_ranges[1::2] - combined_ranges[0::2] + 1)

    print(f"number of fresh ids: {num_fresh_ids}\n")


if __name__ == "__main__":
    scenario = parse_inputs("day_5_test_1")
    solution(scenario)
