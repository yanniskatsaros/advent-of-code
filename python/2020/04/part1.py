import sys
from pathlib import Path
from dataclasses import dataclass
from typing import List, Optional

@dataclass
class Passport:
    byr: str
    iyr: str
    eyr: str
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
        
        return cls(**passport)

def parse_input(text: str) -> List[Passport]:
    lines = text.strip().split('\n\n')

    passports: List[Passport] = []
    for line in lines:
        try:
            passport = Passport.parse(line)
        except TypeError:
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
