import sys
from pathlib import Path
from dataclasses import dataclass
from typing import List

@dataclass
class Policy:
    min: int
    max: int
    char: str

    @classmethod
    def parse(cls, line: str) -> 'Policy':
        parts = line.strip().split()
        min_, max_ = parts[0].strip().split('-')
        min_ = int(min_)
        max_ = int(max_)
        char = parts[1].strip()[0]

        return cls(min_, max_, char)

def parse_password(line: str) -> str:
    parts = line.strip().split()

    return parts[-1].strip()

def validate(password: str, policy: Policy) -> bool:
    count = 0
    for c in password:
        if c == policy.char:
            count += 1
        
        if count > policy.max:
            return False

    return (count >= policy.min) and (count <= policy.max)

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
    path = Path(sys.argv[1]).resolve()
    passwords = read_input(path)
    count = main(passwords)

    print(count)
