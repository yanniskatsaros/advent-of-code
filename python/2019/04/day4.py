import argparse
from typing import Tuple

def validate_password(pwd: int) -> Tuple[bool, int]:
    """
    Return `True` if the given password is valid,
    along with the next password to check.
    """
    text = str(pwd)
    d_prev = int(text[0])
    n = len(text)
    double = False
    for i, d in enumerate(text[1:]):
        d = int(d)
        if d < d_prev:
            i += 1  # correct for offset of 1 while iterating
            next_pwd = text[0:i] + (str(d_prev) * (n-i))
            return False, int(next_pwd)
        else:
            if d == d_prev:
                double = True
            d_prev = d
    next_pwd = pwd + 1
    if double:
        return True, next_pwd
    return False, next_pwd 

def validate_again(pwd: int) -> bool:
    """
    Additional validation required for Part 2
    """
    text = str(pwd)
    d_prev = int(text[0])
    streaks = []
    streak = 1

    for d in text[1:]:
        d = int(d)
        if d == d_prev:
            streak += 1
        else:
            streaks.append(streak)
            streak = 1
        d_prev = d
    
    streaks.append(streak)
    return True if 2 in streaks else False

def count_valid_passwords_1(start: int, end: int) -> int:
    """
    Counts the number of valid passwords in the given
    range of values - Part 1 solution.
    """
    pwd: int = start
    counter: int = 0
    while (pwd <= end):
        keep, pwd = validate_password(pwd)
        if keep:
            counter += 1
    return counter

def count_valid_passwords_2(start: int, end: int) -> int:
    """
    Counts the number of valid passwords in the given
    range of values - Part 2 solution.
    """
    pwd: int = start
    counter: int = 0
    while (pwd <= end):
        keep, next_pwd = validate_password(pwd)

        if keep and validate_again(pwd):
            counter += 1
        pwd = next_pwd

    return counter

def parser_factory():
    """
    Argument parser factory for the CLI
    """
    p = argparse.ArgumentParser(
        description='Day 4 - Advent of Code'
    )
    p.add_argument('start', type=int, help='Start of the range')
    p.add_argument('end', type=int, help='End of the range')

    return p

if __name__ == '__main__':
    parser = parser_factory()
    args = parser.parse_args()

    counts_1 = count_valid_passwords_1(args.start, args.end)
    counts_2 = count_valid_passwords_2(args.start, args.end)
    print('Number of Valid Passwords:')
    print(f'Part 1: {counts_1}')
    print(f'Part 2: {counts_2}')

