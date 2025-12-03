from core import read_file, parse_inputs


def solution(scenario: str):
    input_file = scenario + ".txt"
    input = read_file(input_file)

    # parse input string
    start_stop_strs = [x.split('-') for x in input.split(',')]
    # look for invalid ids in the range defined by each start/stop string
    invalid_ids = []
    for start_stop_str in start_stop_strs:
        start, stop = [int(n) for n in start_stop_str]
        for id in range(start, stop + 1):
            id_str = str(id)
            str_length = len(id_str)
            if str_length % 2 != 0:
                continue
            midpoint = str_length // 2
            if id_str[0:midpoint] == id_str[midpoint:]:
                invalid_ids.append(id)

    print(f"password is {sum(invalid_ids)}")


if __name__ == "__main__":
    scenario = parse_inputs("day_2_test_1")
    solution(scenario)
