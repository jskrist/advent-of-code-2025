# [2025 Advent of Code](https://adventofcode.com/2025)
This is a repository of my 2025 AoC solutions.  I started off doing this in python, and decided later to also complete them in Rust as a way to practice more Rust.

### Notes on Rust Solutions
currently all solutions written in rust are being written as modules that are being called from the main.rs/main() function.  They should be able to be run the top level directory of the repository using a command like the following:
```bash
cargo run -- 1 test_1
```
which follows the formula `cargo run -- <day_number> <input_type>`


### Notes on Python Solutions
python solutions can be run from the top level directory of the repository using a command like the following:
```bash
python python/day_1_part_1.py inputs/day_1_test_1
```
which follows the formula `python <solution file> <input path>`


### Naming Conventions
All solution files follow the following naming convention: `day_<day#>_part_<part#>.<ext>`
e.g. the python solution for day 1 part 1 is in the file: `day_1_part_1.py`

All input files follow the following naming convention: `day_<day#>_<input|test>_<#>.txt`
e.g. the test input for day 1 is in the file: `./inputs/day_1_test_1.txt`

### Expected Solutions
| Day 1 | Part 1 | Part 2 |
|---|---:|---:|
| test_1 | 0 | 2 |
| input_1 | 46 | 1030560 |

| Day 2 | Part 1 | Part 2 |
|---|---:|---:|
| test_1 | 1096386 | 1174163 |
| input_1 | 3523095 | 4360896 |

| Day 3 | Part 1 | Part 2 |
|---|---:|---:|
| test_1 | 392 | 3442797780113 |
| input_1 | 19800 | 199997292572704 |

| Day 4 | Part 1 | Part 2 |
|---|---:|---:|
| test_1 | 30 | 44 |
| input_1 | 3429 | 9129 |
