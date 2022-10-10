import sys
from pathlib import Path
from dataclasses import dataclass
from typing import List, Optional

def validate_year(key: str, value: str) -> int:
    if len(value) != 4:
        raise TypeError(f'expected 4 digit year, got: {value}')

    # try casting to int
    try:
        year = int(value)
    except ValueError:
        raise ValueError(f'expected integer year, got: {year}')

    RULES = {
        'byr': (1920, 2002),
        'iyr': (2010, 2020),
        'eyr': (2020, 2030),
    }
    min_max = RULES.get(key)

    if min_max is None:
        raise TypeError(f'unexpected year key: {key}')

    min_, max_ = min_max
    if (year < min_) or (year > max_):
        raise ValueError(f'{key} outside valid range: {min_, max_}, got: {year}')

    return year

def validate_height(value: str) -> None:
    integers: List[str] = []
    units: str = ''

    for i, c in enumerate(value):
        code = ord(c)
        if (code < 48) or (code > 57):
            units = value[i:]
            break

        integers.append(c)

    height = int(''.join(integers))

    if units not in ('in', 'cm'):
        raise ValueError(f'expected one of: {"in", "cm"}, got: {units}')

    if units == 'cm':
        if (height < 150) or (height > 193):
            msg = f'height in cm outside valid range: (150, 193), got: {height}'
            raise ValueError(msg)

    if units == 'in':
        if (height < 59) or (height > 76):
            msg = f'height in cm outside valid range: (59, 76), got: {height}'
            raise ValueError(msg)

def validate_hair_color(value: str) -> None:
    if not value.startswith('#'):
        raise ValueError('expected "hcl" begins with #')

    # acceptable ascii codes for 0-9 and a-f
    accepted = [ord(str(i)) for i in range(10)]
    for c in 'abcdef':
        accepted.append(ord(c))

    codes = set(accepted)
    remainder = value[1:]

    for c in remainder:
        if ord(c) not in codes:
            raise ValueError(f'unexpected character in "hcl": {c}')

def validate_eye_color(value: str) -> None:
    choices = {
        'amb',
        'blu',
        'brn',
        'gry',
        'grn',
        'hzl',
        'oth',
    }

    if value not in choices:
        raise ValueError(f'expected one of: {choices}, got: {value}')

def validate_passport_id(value: str) -> None:
    if len(value) != 9:
        msg = f'invalid length for "pid" - expected 9 digits, got: {len(value)}'
        raise ValueError(msg)

    # acceptable ascii codes for 0-9
    codes = set([ord(str(i)) for i in range(10)])
    for c in value:
        if ord(c) not in codes:
            raise ValueError(f'unexpected non-digit in "pid": {c}')

@dataclass
class Passport:
    byr: int
    iyr: int
    eyr: int
    hgt: str
    hcl: str
    ecl: str
    pid: str
    cid: Optional[str] = None

    @classmethod
    def parse(cls, line: str) -> 'Passport':
        passport = {}
        tokens = line.strip().split()

        for kv in tokens:
            k, v = kv.strip().split(':')
            passport[k] = v

        # additional validation required here
        for k in ('byr', 'iyr', 'eyr'):
            passport[k] = validate_year(k, passport[k])

        # special validation rules for each field below
        # it's an error if the key doesn't exist so allow KeyError
        validate_height(passport['hgt'])
        validate_hair_color(passport['hcl'])
        validate_eye_color(passport['ecl'])
        validate_passport_id(passport['pid'])
        
        return cls(**passport)

def parse_input(text: str) -> List[Passport]:
    lines = text.strip().split('\n\n')

    passports: List[Passport] = []
    for line in lines:
        try:
            passport = Passport.parse(line)
        except (KeyError, TypeError, ValueError):
            continue

        passports.append(passport)
    
    return passports

if __name__ == '__main__':
    # input: https://adventofcode.com/2020/day/4/input
    path = Path(sys.argv[1]).resolve()

    with open(path, 'r') as f:
        text = f.read()

    # only valid passports will be parsed successfully
    passports = parse_input(text)

    # number of valid passports
    print(len(passports))
