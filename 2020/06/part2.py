import sys
from pathlib import Path
from itertools import chain
from typing import List, Set

def intersection(group: List[str]) -> Set[str]:
    return set.intersection(*[set(g) for g in group])

if __name__ == '__main__':
    # input: https://adventofcode.com/2020/day/6/input
    path = Path(sys.argv[1]).resolve()

    with open(path, 'r') as f:
        text = f.read()
    
    groups = text.strip().split('\n\n')
    lengths = [len(intersection(g.split('\n'))) for g in groups]

    print(sum(lengths))
