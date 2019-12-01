import math
def fuel_needed(fuel):
    return math.floor(fuel/3) - 2

def part_1():
    with open('input.txt') as f:
        return sum((fuel_needed(int(line)) for line in f.readlines()))

def part_2():
    total = 0
    with open('input.txt') as f:
        for fuel in (int(line) for line in f.readlines()):
            while (fuel:= fuel_needed(fuel)) > 0:
                total += fuel
    return total

def rec(fuel):
    new_fuel = fuel_needed(fuel)
    return new_fuel + rec(new_fuel) if new_fuel > 0 else 0

def part_2_rec():
    with open('input.txt') as f:
        return sum(rec(int(line)) for line in f.readlines())
    


if __name__ == "__main__":
    print(part_1())
    print(part_2())
    print(part_2_rec())
