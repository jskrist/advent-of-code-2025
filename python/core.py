import os
from pathlib import Path
import sys


def read_file(filename: str | os.PathLike) -> str:
    filename = Path(filename)
    with open(filename, "r") as file:
        return file.read().strip()


def parse_inputs(default):
    if len(sys.argv) > 1:
        scenario = sys.argv[1]
    else:
        scenario = default
    return scenario
