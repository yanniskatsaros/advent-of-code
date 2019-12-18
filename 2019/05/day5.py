import os, argparse
from typing import List, Optional, NamedTuple, Tuple

def get_input(file: str) -> List[int]:
    """Gets the input for the problem"""
    with open(file, 'r') as f:
        prog: List[str] = f.read().split(',')

    return [int(i) for i in prog]

def parser_factory():
    p = argparse.ArgumentParser('day5', description='Day 5 - Intcode Computer')
    p.add_argument('n', type=int, help='Input for the Intcode computer')

    return p

class Mode(NamedTuple):
    p1: int
    p2: int

class Instruction(NamedTuple):
    opcode: int
    mode: Mode

def parse_instruction(instruction) -> Instruction:
    ix = str(instruction)
    n = len(ix)
    opcode = int(ix[-2:])

    if n <= 2:
        return Instruction(opcode, Mode(0, 0))
    elif n == 3:
        return Instruction(opcode, Mode(int(ix[0]), 0))
    elif n == 4:
        return Instruction(opcode, Mode(int(ix[1]), int(ix[0])))
    else:
        return Instruction(99, Mode(0, 0))

class Intcode:
    def __init__(self, memory: List[int]):
        self.mem: List[int] = memory
        self.i = 0  # instruction pointer
        self.stdin = []
        self.stdout = []
        self.end = False  # flag that program has ended running

    def run(self, stdin: int) -> int:
        """
        Run the Intcode program.

        Parameters
        ----------
        stdin : int
            Input instruction passed into the program

        Returns
        -------
        output : int
            The output instruction the program returns
            after it has completed its run
        
        """
        self.stdin.append(stdin)

        while not self.end:
            instruction: int = self.mem[self.i]
            ix: Instruction = parse_instruction(instruction)
            self._instruction(ix)

    def _mode_value(self, i: int, m: int) -> int:
        """
        Return a value according to the parameter
        mode given.

        Parameters
        ----------
        i : int
            Memory pointer or literal value

        m : int
            Parameter mode
                - 0: value at memory address `i`
                - 1: the literal value of `i`

        Returns
        -------
        int
        """
        addr = self.mem[i]
        if m == 0:
            return self.mem[addr]
        return m

    def _instruction(self, ix: Instruction) -> None:
        """
        Perform the instruction with the given parameter modes
        """

        def _add(mode: Mode) -> None:
            a = self._mode_value(self.i+1, mode.p1)
            b = self._mode_value(self.i+2, mode.p2)
            c = self.mem[self.i+3]
            self.mem[c] = a + b
            self.i += 4

        def _mult(mode: Mode) -> None:
            a = self._mode_value(self.i+1, mode.p1)
            b = self._mode_value(self.i+2, mode.p2)
            c = self.mem[self.i+3]
            self.mem[c] = a * b
            self.i += 4

        def _store(mode: Mode) -> None:
            addr = self.mem[self.i+1]
            self.mem[addr] = self.stdin.pop()
            self.i += 2

        def _emit(mode: Mode) -> None:
            addr: int = self.mem[self.i+1]
            v: int = self.mem[addr]
            self.stdout.append((self.i, v))
            self.i += 2

        def _end(mode: Mode) -> None:
            self.i += 1
            self.end = True
            print(self.stdout)
            return

        instruction_map = {
            1: _add,
            2: _mult,
            3: _store,
            4: _emit,
            99: _end
        }

        op = ix.opcode
        mode = ix.mode
        return instruction_map[op](mode)

if __name__ == '__main__':
    parser = parser_factory()
    args = parser.parse_args()

    memory = get_input('input.txt')
    computer = Intcode(memory)
    computer.run(args.n)
