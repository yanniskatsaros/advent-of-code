import sys
from pathlib import Path
from dataclasses import dataclass
from typing import List, Optional

@dataclass
class BinarySearchTree:
    data: int
    left: Optional['BinarySearchTree'] = None
    right: Optional['BinarySearchTree'] = None

def build_bst(items: List[int]) -> BinarySearchTree:
    n = len(items)

    # sort first to create the most balanced tree
    items = sorted(items)

    if n <= 1:
        return BinarySearchTree(items[0])

    # the index to split on
    i = n // 2
    data = items[i]
    left = build_bst(items[:i])
    right = build_bst(items[i:])

    return BinarySearchTree(data, left, right)

def parse_row_code(code: str) -> int:
    k = len(code)
    # number of possible nodes
    n = 2 ** k
    items = list(range(n))

    # build a binary search tree to find the index
    tree = build_bst(items)

    for c in code:
        if c not in ('F', 'B'):
            raise ValueError(f'unexpected character in row code: {c}')

        if c == 'F':
            tree = tree.left
        
        if c == 'B':
            tree = tree.right

    return tree.data

def parse_col_code(code: str) -> int:
    k = len(code)
    # number of possible nodes
    n = 2 ** k
    items = list(range(n))

    # build a binary search tree to find the index
    tree = build_bst(items)

    for c in code:
        if c not in ('R', 'L'):
            raise ValueError(f'unexpected character in row code: {c}')

        if c == 'L':
            tree = tree.left
        
        if c == 'R':
            tree = tree.right

    return tree.data

def parse_boarding_pass(code: str) -> int:
    """Generate the seat ID from the boarding pass code"""
    n = len(code)

    if n != 10:
        msg = f'invalid boarding pass length, expected 10 got: {n}'
        raise ValueError(msg)

    row_code = code[:7]
    col_code = code[7:]

    row = parse_row_code(row_code)
    col = parse_col_code(col_code)

    # magic recipe
    return (row * 8) + col

if __name__ == '__main__':
    # input: https://adventofcode.com/2020/day/5/input
    path = Path(sys.argv[1]).resolve()

    with open(path, 'r') as f:
        text = f.read()

    codes = text.strip().split('\n')
    ids = [parse_boarding_pass(c) for c in codes]

    # find the missing id in the sequence
    ids = sorted(ids)
    cur = ids[0]
    for i in ids[1:]:
        if i != cur + 1:
            cur += 1
            break
        cur = i

    print(cur)