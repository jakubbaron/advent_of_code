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


if __name__ == "__main__":
    print(part_1())
    print(part_2())
