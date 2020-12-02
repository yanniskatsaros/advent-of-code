import sys
from pathlib import Path
from dataclasses import dataclass
from typing import List

@dataclass
class Policy:
    a: int
    b: int
    char: str

    @classmethod
    def parse(cls, line: str) -> 'Policy':
        parts = line.strip().split()
        a, b = parts[0].strip().split('-')
        a, b = int(a), int(b)
        char = parts[1].strip()[0]

        return cls(a, b, char)

def parse_password(line: str) -> str:
    parts = line.strip().split()

    return parts[-1].strip()

def validate(password: str, policy: Policy) -> bool:
    # indexes
    i = policy.a - 1
    j = policy.b - 1

    # logical XOR
    a = password[i] == policy.char
    b = password[j] == policy.char
    return (a and not b) or (not a and b)

def main(passwords: List[str]) -> int:
    count = 0

    for line in passwords:
        policy = Policy.parse(line)
        password = parse_password(line)

        if validate(password, policy):
            count += 1
        
    return count

def read_input(path: Path) -> List[str]:
    with open(path, 'r') as f:
        lines = f.readlines()

    return lines

if __name__ == '__main__':
    # input: https://adventofcode.com/2020/day/2/input
    path = Path(sys.argv[1]).resolve()
    passwords = read_input(path)
    count = main(passwords)

    print(count)
