import os
from typing import NamedTuple, Tuple, List, Optional
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

    @classmethod
    def origin(cls):
        return cls(0, 0)

class Line(NamedTuple):
    a: Point
    b: Point

    def __eq__(self, other):
        return (self.a == other.a) and (self.b == other.b)

    def __len__(self) -> int:
        return self.length()

    def x_range(self) -> range: 
        """The mathematical domain of x-values for the line"""
        bounds = sorted([self.a.x, self.b.x])
        return range(bounds[0], bounds[1] + 1) 

    def y_range(self) -> range: 
        """The mathematical range of y-values for the line"""
        bounds = sorted([self.a.y, self.b.y])
        return range(bounds[0], bounds[1] + 1)

    def length(self) -> int:
        """Computes the length of the 1D line"""
        return len(self.x_range()) + len(self.y_range()) - 2

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
    def __init__(self, lines: List[Line]):
        """
        Create a Wire object from an iterable of Lines

        Parameters
        ----------
        lines : List[Line]
            An iterable of each line in the wire

        Examples
        --------
        >>> lines = [
         Line(a=Point(x=0, y=0), b=Point(x=0, y=7))
         Line(a=Point(x=0, y=7), b=Point(x=6, y=7))
         Line(a=Point(x=6, y=7), b=Point(x=6, y=3))
         Line(a=Point(x=6, y=3), b=Point(x=2, y=3))]
        ]
        >>> wire = Wire('U7,R6,D4,L4')
        >>> print(wire)
        [Line(a=Point(x=0, y=0), b=Point(x=0, y=7))
         Line(a=Point(x=0, y=7), b=Point(x=6, y=7))
         Line(a=Point(x=6, y=7), b=Point(x=6, y=3))
         Line(a=Point(x=6, y=3), b=Point(x=2, y=3))]

        """
        self.lines = lines
        self.cumlength = list(accumulate([len(l) for l in lines]))
    

    @classmethod
    def from_string(cls, wire: str):
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
        >>> wire = Wire.from_string('U7,R6,D4,L4')
        >>> print(wire)
        [Line(a=Point(x=0, y=0), b=Point(x=0, y=7))
         Line(a=Point(x=0, y=7), b=Point(x=6, y=7))
         Line(a=Point(x=6, y=7), b=Point(x=6, y=3))
         Line(a=Point(x=6, y=3), b=Point(x=2, y=3))]

        """
        wire_path = wire.strip().split(',')
        points = [Point(0, 0)]
        points.extend([cls._parse_wire_path(w.strip()) for w in wire_path])
        coords = list(accumulate(points))
        lines = [Line(coords[i-1], coords[i]) for i in range(1, len(coords))]
        return cls(lines)
   
    def __repr__(self):
        return str(self.lines)

    def __iter__(self):
        return iter(self.lines)

    def __getitem__(self, key):
        if isinstance(key, int):
            return self.lines[key]
        elif isinstance(key, slice):
            return Wire(self.lines[key])
        else:
            raise KeyError('Invalid key for type: Wire')

    def __len__(self) -> int:
        return self.cumlength[-1]

    @staticmethod
    def _parse_wire_path(s: str) -> Point:
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

class Intersection(NamedTuple):
    point: Point
    steps: int

def get_wires(file: str) -> Tuple[Wire, Wire]:
    with open(file, 'r') as f:
        wires = f.read().strip().split('\n')

    return Wire.from_string(wires[0]), Wire.from_string(wires[1])

def main_part_1() -> int:
    wire_a, wire_b = get_wires('input.txt')
    origin: Point = Point.origin()

    intersections: List[Point] = []
    for la in wire_a:
        for lb in wire_b:
            point: Point = la.intersect(lb)
            if (point is not None) and (point != origin):
                intersections.append(point)

    min_distance = min([i.manhattan_distance() for i in intersections])
    return min_distance 

def main_part_2() -> int:
    wire_a, wire_b = get_wires('input.txt')
    origin: Point = Point.origin()
    
    intersections: List[Intersection] = []
    for ia, la in enumerate(wire_a):
        for ib, lb in enumerate(wire_b):
            point: Point = la.intersect(lb)
            if (point is None) or (point == origin):
                continue

            la_prev: Line = wire_a[ia-1]
            lb_prev: Line = wire_b[ib-1]

            if la.is_vertical():
                a_len = abs(point.y - la_prev.b.y)
                b_len = abs(point.x - lb_prev.b.x)
            else:
                a_len = abs(point.x - la_prev.b.x)
                b_len = abs(point.y - lb_prev.b.y)
            
            total_a = a_len + len(wire_a[0:ia])
            total_b = b_len + len(wire_b[0:ib])
            inter = Intersection(point, total_a + total_b)
            intersections.append(inter)

    min_steps = min(intersections, key=lambda x: x.steps)
    return min_steps.steps

if __name__ == '__main__':
    min_distance = main_part_1()
    min_steps = main_part_2()

    print(f'Min. Manhattan Distance: {min_distance}')
    print(f'Min. Total Steps: {min_steps}')
