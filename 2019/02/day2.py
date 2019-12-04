import os
from typing import List, Optional

def get_input(file: str) -> List[int]:
    """Gets the input for the problem"""
    with open(file, 'r') as f:
        prog: List[str] = f.read().split(',')

    return [int(i) for i in prog]

def operator(code: int, a: int, b: int) -> Optional[int]:
    """
    Performs an addition, multiplication, or nothing
    according to the provided `code`.
    """
    ops = {
        1: a + b,
        2: a * b,
        99: None
    }
    return ops[code]

def intcode_step(program: List[int], i: int) -> List[int]:
    """
    Performs a single calculation step on the program
    given the index of the opcode
    """
    if (op := program[i]) == 99:
        return program

    ia, ib, ic = program[i+1], program[i+2], program[i+3]
    a, b = program[ia], program[ib]
    result = operator(op, a, b)
    program[ic] = result
    
    return program

def main(file: str) -> None:
    """
    Restore the gravity assist program
    and run the "Intcode" computer.
    """
    prog = get_input(file)
    n = len(prog)

    # first, restore the gravity assist program
    prog[1] = 12
    prog[2] = 2

    for i in range(0, n, 4):
        op = prog[i]
        if op == 99:
            break
        prog = intcode_step(prog, i)

    print(f'Program position 0: {prog[0]}')

if __name__ == '__main__':
    main('input.txt')
