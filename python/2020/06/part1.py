import sys
from pathlib import Path
from itertools import chain
from typing import List, Set

def unique_chars(group: List[str]) -> Set[str]:
    return set(x for x in chain(*group))

if __name__ == '__main__':
    # input: https://adventofcode.com/2020/day/6/input
    path = Path(sys.argv[1]).resolve()

    with open(path, 'r') as f:
        text = f.read()
    
    groups = text.strip().split('\n\n')
    lengths = [len(unique_chars(g.split('\n'))) for g in groups]

    print(sum(lengths))
