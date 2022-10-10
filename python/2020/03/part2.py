import sys
from pathlib import Path
from typing import List, TypeVar
from itertools import accumulate
from operator import mul

T = TypeVar('T')

class Indexer:
    def __init__(self, items: List[T]):
        self.items = items
        self.n = len(items)

    def __getitem__(self, idx: int):
        if isinstance(idx, slice):
            raise TypeError('slice indexer is unsupported')

        i = idx % self.n
        return self.items[i]

    def __repr__(self) -> str:
        return repr(self.items)

    def __str__(self) -> str:
        return str(self.items)

def parse_input(text: str) -> List[Indexer]:
    """
    Parse the input grid converting empty
    spaces, "." -> 0, and trees, "#" -> 1
    """
    lines = text.strip().split('\n')
    
    mapper = {
        '.': 0,
        '#': 1,
    }

    grid = [list(map(mapper.get, l)) for l in lines]

    return [Indexer(r) for r in grid]

def count(grid: List[Indexer], right: int, down: int) -> int:
    """
    Count the number of "trees" encountered when traversing
    the slope of the given `grid` for a specified step size
    of `right` steps and `down` steps.
    """
    count = 0
    i = 0
    j = 0

    while True:
        i += right
        j += down
        try:
            val = grid[j][i]
        except IndexError:
            return count
        
        if val == 1:
            count += 1
    
    return count

if __name__ == '__main__':
    # input: https://adventofcode.com/2020/day/3/input
    path = Path(sys.argv[1]).resolve()

    with open(path, 'r') as f:
        text = f.read()

    grid = parse_input(text)

    slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ]
    counts = [count(grid, right, down) for right, down in slopes]
    result = list(accumulate(counts, func=mul))

    print(result[-1])
