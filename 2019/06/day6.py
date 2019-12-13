import os, argparse
from typing import List, Tuple, Any, Optional

class Queue:
    def __init__(self, items: Optional[List[Any]]=None):
        if items:
            self.items = items
        else:
            self.items = []

    def __repr__(self) -> str:
        return f'{self.items}'

    def __str__(self) -> str:
        return self.__repr__()

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def put(self, item: Any):
        self.items.append(item)

    def get(self) -> Any:
        if self.is_empty():
            raise RuntimeError('Empty queue!')
        item: Any = self.items[0]
        self.items = self.items[1:]

        return item

class Planet:
    def __init__(self, name: str):
        self.name = name
        self.parent = None
        self.children: List[Planet] = []

    def __repr__(self) -> str:
        return f"Planet('{self.name}')"

    def __str__(self) -> str:
        return self.__repr__()

    def __eq__(self, other) -> bool:
        return self.name == other.name

    @classmethod
    def parse_orbit(cls, orbit: str) -> Tuple:
        names = orbit.strip().split(')')

        return cls(names[0]), cls(names[1]) 

class OrbitTree:
    """
    A special type of tree representing orbits of Planets
    """
    def __init__(self):
        self.planets = {}
    
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

class OrbitGraph:
    """
    A graph representing the map of planet orbits
    """
    def __init__(self):
        self.planets = {}

    def add_orbit(self, a: str, b: str):
        if a in self.planets:
            a_adj = self.planets[a]
        else:
            a_adj = []
        
        if b in self.planets:
            b_adj = self.planets[b]
        else:
            b_adj = []

        # A is adjacent to B and vice-versa
        b_adj.append(a)
        a_adj.append(b)

        self.planets[a] = a_adj
        self.planets[b] = b_adj

    def min_orbital_transfers(self, a: str, b: str) -> int:
        """
        Find the minimum number of transfers required
        to move from the orbit of planet `a` to the
        orbit of planet `b`.
        """
        # use breadth first search to get the path
        p: Optional[Planet] = planet_bfs(self, a, b)
        if p is None:
            return None

        path = []
        parent = p.parent
        while parent is not None:
            path.append(parent)
            parent = parent.parent

        # -2 because we want the moves to each
        # planet's parent, not to the actual planet
        return len(path) - 2

def get_input(file: str) -> List[str]:
    with open(file, 'r') as f:
        lines = f.read().strip().split('\n')

    return lines

def build_tree(orbits: List[str]) -> OrbitTree:
    tree = OrbitTree()

    for orbit in orbits:
        b, a = Planet.parse_orbit(orbit)
        tree.add_orbit(a, b)  # read as: A orbits B

    return tree

def build_graph(orbits: List[str]) -> OrbitGraph:
    graph = OrbitGraph()

    for orbit in orbits:
        # b: Planet, a: Planet
        b, a = Planet.parse_orbit(orbit)
        graph.add_orbit(a.name, b.name)

    return graph

def planet_bfs(graph: OrbitGraph, start: str, search: str) -> Optional[Planet]:
    """Breadth First Search, for Planets (for Humans)"""
    goal = Planet(search)
    start = Planet(start)

    # will keep track of all visited Planets in the search
    visited: List[Planet] = [start]

    # used for simulating recursion with iteration
    q = Queue() 
    q.put(start)

    while not q.is_empty():
        v: Planet = q.get()
        if v == goal:
            return v

        # all planets adjacent to `v` in the graph
        adj: List[str] = graph.planets[v.name]
        for w in adj:
            w = Planet(w)
            if w not in visited:
                visited.append(w)
                w.parent = v
                q.put(w)
    # if no planet is found
    return None

def part_1() -> int:
    """
    Find the total number of orbits
    (direct and indirect)
    """
    orbits: List[str] = get_input('input.txt')
    tree: OrbitTree = build_tree(orbits)
    total_orbits: int = tree.total_orbits()

    return total_orbits

def part_2() -> int:
    """
    Find the minimum number of orbital transfers
    required to move from the planet YOU are
    orbiting and SAN is orbiting.
    """
    orbits: List[str] = get_input('input.txt')
    graph: OrbitGraph = build_graph(orbits)
    min_transfers: int = graph.min_orbital_transfers('YOU', 'SAN')

    return min_transfers

if __name__ == '__main__':
    total_orbits = part_1()
    print(f'Total Orbits: {total_orbits}')

    min_transfers = part_2()
    print(f'Minimum Orbital Transfers: {min_transfers}')
