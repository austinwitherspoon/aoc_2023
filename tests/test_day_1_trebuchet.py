import pytest

from aoc_2023 import day_1_trebuchet


def test_example_1():
    """First example provided by AOC"""
    EXAMPLE_INPUT = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""
    assert day_1_trebuchet.get_calibration_value(EXAMPLE_INPUT) == 142


def test_example_2():
    """second example provided by AOC"""
    EXAMPLE_INPUT = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""
    assert day_1_trebuchet.get_calibration_value(EXAMPLE_INPUT) == 281


@pytest.mark.parametrize(
    "line,expected",
    [
        ("1abc2", 1),
        ("pqr3stu8vwx", 3),
        ("a1b2c3d4e5f", 1),
        ("treb7uchet", 7),
        ("two1nine", 2),
        ("eightwothree", 8),
        ("abcone2threexyz", 1),
        ("xtwone3four", 2),
        ("4nineeightseven2", 4),
        ("zoneight234", 1),
        ("7pqrstsixteen", 7),
    ],
)
def test_get_first_digit(line: str, expected: int):
    """Test that our function always gets the correct first digit of the line."""
    assert day_1_trebuchet.get_first_digit(line) == expected


@pytest.mark.parametrize(
    "line,expected",
    [
        ("1abc2", 2),
        ("pqr3stu8vwx", 8),
        ("a1b2c3d4e5f", 5),
        ("treb7uchet", 7),
        ("two1nine", 9),
        ("eightwothree", 3),
        ("abcone2threexyz", 3),
        ("xtwone3four", 4),
        ("4nineeightseven2", 2),
        ("zoneight234", 4),
        ("7pqrstsixteen", 6),
        ("3tgsppcfpk", 3),
    ],
)
def test_get_last_digit(line: str, expected: int):
    """Test that our function always gets the correct last digit of the line."""
    assert day_1_trebuchet.get_last_digit(line) == expected


def test_actual_puzzle_input():
    """Test the actual puzzle provided by advent of code."""
    with open("resources/day_1/puzzle_input.txt") as f:
        puzzle_input = f.read()
    assert day_1_trebuchet.get_calibration_value(puzzle_input) == 53855
