import os
from typing import NamedTuple, Tuple
from itertools import accumulate

class Point(NamedTuple):
    x: int
    y: int

    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)

    def __sub__(self, other):
        return Point(self.x - other.x, self.y - other.y)

    def __neg__(self):
        return Point(-self.x, -self.y) 

class Wire:
    def __init__(self, wire: str):
        """
        Create a Wire object from a comma delimited
        string specifying the wire path (coordinates).

        The "center" of the imaginary grid (ie: coordinate system)
        used is assumed to be at Point(x=0, y=0).

        Parameters
        ----------
        wire : str
            String of comma delimited values

        Examples
        --------
        >>> wire_str = 'U7,R6,D4,L4'
        >>> wire = Wire(wire_str)
        >>> print(wire)
        >>> [Point(0, 7), Point(6, 7), Point(6, 3), Point(2, 3)]
        """
        wire_path = wire.strip().split(',')
        points = [self._parse_wire_path(w) for w in wire_path]
        self.coords = list(accumulate(points))
    
    def __repr__(self):
        return str(self.coords)

    def _parse_wire_path(self, s: str) -> Point:
        """
        Parses a wire path string such as "U7" or "L4"
        """
        direction = s[0].upper()
        val = int(s[1:])
        
        DIR_MAP = {
            'U': Point(0, val),
            'R': Point(val, 0),
            'D': Point(0, -val),
            'L': Point(-val, 0)
        }
        return DIR_MAP[direction]

def get_wires(file: str) -> Tuple[Wire, Wire]:
    with open(file, 'r') as f:
        wires = f.read().strip().split('\n')

    return Wire(wires[0]), Wire(wires[1])
