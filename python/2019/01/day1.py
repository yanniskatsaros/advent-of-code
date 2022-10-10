import os
from typing import List

def get_input(file: str) -> List[int]:
    """
    Gets the input for the problem
    """
    return [int(i) for i in open(file, 'r')]

def calc_fuel(mass: int) -> int:
    """
    Calculates the fuel requirement from the mass
    """
    return (mass // 3) - 2

def recursive_fuel(mass: int) -> int:
    """
    Calculates the fuel requirement from the mass
    taking into account that the fuel itself
    has its own mass etc.
    """
    if mass <= 0:
        return 0

    # yay for walrus operator!
    if (fuel := (mass // 3) - 2) < 0:
        fuel = 0

    return fuel + recursive_fuel(fuel)

if __name__ == '__main__':
    mass = get_input('input.txt')
    fuel = [calc_fuel(m) for m in mass]
    extra_fuel = [recursive_fuel(m) for m in mass]

    print(f'Total Fuel Required: {sum(fuel)}')
    print(f'Total (Recursive) Fuel Required: {sum(extra_fuel)}')
