def part_1(data: list[str]):
    sum = 0
    for line in data:
        first_digit = 0
        second_digit = 0
        for char in line:
            try:
                first_digit = int(char)
                break
            except:
                pass
        for char in reversed(line):
            try:
                second_digit = int(char)
                break
            except:
                pass
        sum += first_digit * 10 + second_digit
    return sum



def part_2(data: list[str]):
    def to_digit(d: str) -> int:
        if len(d) > 1:
            return{
                "one": 1,
                "two": 2,
                "three": 3,
                "four": 4,
                "five": 5,
                "six": 6,
                "seven": 7,
                "eight": 8,
                "nine": 9
            }[d]
        else:
            return int(d)
    sum = 0
    for line in data:
        left_indexes = {}
        right_indexes = {}
        for string_digit in ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]:
            left_indexes[string_digit] = line.find(string_digit)
            right_indexes[string_digit] = line.rfind(string_digit)
        index_left = {v: k for k,v in left_indexes.items()}
        min_index = min(set(index_left.keys()) - {-1})
        index_right = {v: k for k,v in right_indexes.items()}
        max_index = max(set(index_right.keys()) - {-1})
        sum += to_digit(index_left[min_index]) * 10 + to_digit(index_right[max_index])
    return sum


test_input = """
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""
print(part_1(test_input))
with open('01_input.txt', 'r') as f:
    print(part_1(f.readlines()))
with open('01_input.txt', 'r') as f:
    print(part_2(f.readlines()))

