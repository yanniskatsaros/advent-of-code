import os, argparse
from typing import List, Tuple, Generator

class Planet:
    def __init__(self, name: str):
        self.name = name
        self.parent = None
        self.children: List[Planet] = []

    def __repr__(self) -> str:
        return f"Planet('{self.name}')"

    def __str__(self) -> str:
        return self.__repr__()

    @classmethod
    def parse_orbit(cls, orbit: str) -> Tuple:
        names = orbit.strip().split(')')

        return cls(names[0]), cls(names[1]) 

class OrbitGraph:
    """
    A special type of DAG representing orbits of Planets
    """
    def __init__(self):
        self.planets = {
            'COM': Planet('COM')
        }
    
    def add_orbit(self, a: Planet, b: Planet):
        """
        Specify that planet `a` orbits planet `b`
        on the orbit map.
        """
        if a.name in self.planets:
            a = self.planets[a.name]

        if b.name in self.planets:
            b = self.planets[b.name]

        # register that B is the parent of A
        a.parent = b
        b.children.append(a)
        self.planets[a.name] = a
        self.planets[b.name] = b

    def total_orbits(self) -> int:
        """
        Compute the total number of orbits
        including both direct and indirect
        """
        i = 0
        for p in self.planets.values():
            parent: Optional[Planet] = p.parent
            while parent is not None:
                i += 1
                parent = parent.parent
        return i

def get_input(file: str) -> List[str]:
    with open(file, 'r') as f:
        lines = f.read().strip().split('\n')

    return lines

def build_graph(orbits: List[str]) -> OrbitGraph:
    graph = OrbitGraph()

    for orbit in orbits:
        b, a = Planet.parse_orbit(orbit)
        graph.add_orbit(a, b)  # read as: A orbits B

    return graph

if __name__ == '__main__':
    orbits = get_input('input.txt')
    graph = build_graph(orbits)
    total_orbits = graph.total_orbits()

    print(f'Total Orbits: {total_orbits}')
