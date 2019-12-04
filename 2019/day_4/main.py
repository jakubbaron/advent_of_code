# --- Day 4: Secure Container ---
# You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.

# However, they do remember a few key facts about the password:

# It is a six-digit number.
# The value is within the range given in your puzzle input.
# Two adjacent digits are the same (like 22 in 122345).
# Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
# Other than the range rule, the following are true:

# 111111 meets these criteria (double 11, never decreases).
# 223450 does not meet these criteria (decreasing pair of digits 50).
# 123789 does not meet these criteria (no double).
# How many different passwords within the range given in your puzzle input meet these criteria?

# Your puzzle input is 372304-847060.
from collections import Counter


def not_decreasing(numbers):
    return not any((numbers[i + 1] < numbers[i] for i in range(len(numbers) - 1)))


def has_double_digits(numbers):
    return any((numbers[i] == numbers[i + 1] for i in range(len(numbers) - 1)))


def not_part_of_larger_group(numbers):
    return any(v == 2 for v in Counter(numbers).values())


def part_1(r, l):
    return sum(
        (
            all((not_decreasing(s), has_double_digits(s)))
            for s in (str(c) for c in range(r, l + 1))
        )
    )


def part_2(r, l):
    return sum(
        (
            all((not_decreasing(s), has_double_digits(s), not_part_of_larger_group(s)))
            for s in (str(c) for c in range(r, l + 1))
        )
    )


if __name__ == '__main__':
    r = 372304
    l = 847060
    print(part_1(r, l))
    print(part_2(r, l))
