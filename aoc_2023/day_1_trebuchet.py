"""Advent of code, day 1.
https://adventofcode.com/2023/day/1

Day 1: Trebuchet?!
"""
import sys
from pathlib import Path
import argparse
import logging

LOGGER = logging.getLogger(__name__)
LOGGER.addHandler(logging.StreamHandler(sys.stderr))
LOGGER.setLevel(logging.DEBUG)

# for our purposes, these are all numbers we care about, because
# from these we can determine the start or end digit of any written number
NUMBERS = {
    "0": 0,
    "1": 1,
    "2": 2,
    "3": 3,
    "4": 4,
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9,
    "zero": 0,
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
    "ten": 10,
    "eleven": 11,
    "twelve": 12,
    "thirteen": 13,
    "fourteen": 14,
    "fifteen": 15,
    "sixteen": 16,
    "seventeen": 17,
    "eighteen": 18,
    "nineteen": 19,
    "twenty": 20,
    "thirty": 30,
    "forty": 40,
    "fifty": 50,
    "sixty": 60,
    "seventy": 70,
    "eighty": 80,
    "ninety": 90,
    "hundred": 100,
}

REVERSED_WORDS = {k[::-1]: v for k, v in NUMBERS.items()}


def get_first_digit(line: str) -> int:
    for i in range(len(line)):
        char = line[i]
        if char.isdigit():
            return int(char)
        substr = line[: i + 1]
        for number_word, value in reversed(NUMBERS.items()):
            if len(substr) >= len(number_word) and number_word in substr:
                return int(str(value)[0])
    raise ValueError(f"Could not find first digit in line: {line}")


def get_last_digit(line: str) -> int:
    for i in range(len(line) - 1, -1, -1):
        char = line[i]
        if char.isdigit():
            return int(char)
        substr = line[i:][::-1]
        for number_word, value in reversed(REVERSED_WORDS.items()):
            if len(substr) >= len(number_word) and number_word in substr:
                return int(str(value)[-1])
    raise ValueError(f"Could not find last digit in line: {line}")


def get_calibration_value(document: str) -> int:
    """Given a document, return the sum of all
    calibration values extracted from the document."""
    LOGGER.info("Getting calibration value from document.")
    LOGGER.debug(f"Document: {document}")
    numbers = []
    for line in document.splitlines():
        LOGGER.debug(f"Line: {line}")
        first_digit = get_first_digit(line)
        last_digit = get_last_digit(line)
        LOGGER.debug(f"First digit: {first_digit}")
        LOGGER.debug(f"Last digit: {last_digit}")
        numbers.append(int(str(first_digit) + str(last_digit)))

    LOGGER.debug(f"Numbers: {numbers}")

    calibration_value = sum(numbers)
    LOGGER.info(f"Calibration value: {calibration_value}")

    return calibration_value


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Day 1 of Advent of Code 2023")
    parser.add_argument(
        "document",
        type=Path,
        help="Path to a document containing calibration values.",
    )

    args = parser.parse_args()

    with args.document.open() as document:
        print(get_calibration_value(document.read()))
