from math import prod
def part_1(loaded_cubes: dict[str, int], data: list[str]) -> int:
    sum = 0
    for line in data:
        game_part, rest = line.split(": ")
        _, game_id = game_part.split(" ")
        game_id = int(game_id)
        cube_parts = list(map(lambda l: list(map(lambda x: x.strip(), l.split(","))), rest.split(";")))
        invalid = False
        for drawn_set in cube_parts:
            for cube in drawn_set:
                number_of_dice, colour = cube.split(" ")
                if loaded_cubes[colour] < int(number_of_dice):
                    invalid = True
        if not invalid:
            sum += game_id
    return sum

def part_2(data: list[str]) -> int:
    sum = 0
    for line in data:
        game_part, rest = line.split(": ")
        _, game_id = game_part.split(" ")
        cube_parts = list(map(lambda l: list(map(lambda x: x.strip(), l.split(","))), rest.split(";")))
        min_cubes = {}
        for drawn_set in cube_parts:
            for cube in drawn_set:
                number_of_dice, colour = cube.split(" ")
                min_cubes[colour] = max(min_cubes.get(colour, int(number_of_dice)), int(number_of_dice))
        sum += prod(min_cubes.values())



    return sum

test_input = [
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
]
test_loaded_cubes = {"red": 12, "green": 13, "blue": 14}
print(part_1(test_loaded_cubes, test_input))
with open("02_input.txt", "r") as f:
    print(part_1({"red": 12, "green": 13, "blue": 14}, f.readlines()))
print(part_2(test_input))
with open("02_input.txt", "r") as f:
    print(part_2(f.readlines()))
