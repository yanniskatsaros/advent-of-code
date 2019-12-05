import os
from typing import NamedTuple, Tuple, Optional
from itertools import accumulate

class Point(NamedTuple):
    x: int
    y: int

    def __eq__(self, other) -> bool:
        return (self.x == other.x) and (self.y == other.y)

    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)

    def __sub__(self, other):
        return Point(self.x - other.x, self.y - other.y)

    def __neg__(self):
        return Point(-self.x, -self.y)

    def manhattan_distance(self) -> int:
        """
        Computes the Manhattan Distance from the
        point to the origin, Point(0, 0).
        """
        return abs(self.x) + abs(self.y)

class Line(NamedTuple):
    a: Point
    b: Point

    def __eq__(self, other):
        return (self.a == other.a) and (self.b == other.b)

    def x_range(self) -> range: 
        """The mathematical domain of x-values for the line"""
        bounds = sorted([self.a.x, self.b.x])
        return range(bounds[0], bounds[1] + 1) 

    def y_range(self) -> range: 
        """The mathematical range of y-values for the line"""
        bounds = sorted([self.a.y, self.b.y])
        return range(bounds[0], bounds[1] + 1) 

    def is_vertical(self) -> bool:
        """
        Returns `True` if the line is vertical
        `False` if its horizontal
        """
        if self.a.x == self.b.x:
            return True 
        return False 

    def intersect(self, other) -> Optional[Point]:
        """
        Determines if two lines have an intersection.
        
        Parameters
        ----------
        other : Line
            The other, possibly intersecting line

        Returns
        -------
        result : Optional[Point]
            If an intersection exists, return the
            coordinates result as a `Point`.
            Otherwise, return `None` if no intersection
            is found between the two lines.

        """
        def _intersect_vertical() -> Optional[Point]:
            # self: vertical line
            # other: horizontal line
            x: int = self.a.x
            y: int = other.a.y

            if (x in other.x_range()) and (y in self.y_range()): 
                return Point(x, y)
            return None

        def _intersect_horizontal() -> Optional[Point]:
            # self: horizontal line
            # other: vertical line
            x: int = other.a.x
            y: int = self.a.y

            if (x in self.x_range()) and (y in other.y_range()):
                return Point(x, y)
            return None

        # equal lines have an infinite number of intersections
        if self == other:
            return None

        if self.is_vertical():
            return _intersect_vertical()

        return _intersect_horizontal()

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
        points = [self._parse_wire_path(w.strip()) for w in wire_path]
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
