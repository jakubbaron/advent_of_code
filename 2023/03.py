def part_1(data: list[str]):
    stripped = []
    for d in data:
        stripped.append(d.strip())
    data = stripped
    data_to_fill = []
    for i in range(len(data)):
        curr = []
        for j in range(len(data[i])):
            curr.append(None)
        data_to_fill.append(curr)
    assert len(data) == len(data_to_fill)
    for dd in range(len(data)):
        assert len(data[dd]) == len(data_to_fill[dd])

    id_to_no= {}
    part_id = 1
    for i, line in enumerate(data):
        current_digit = []
        for j, ch in enumerate(line):
            if ch in {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"}:
                current_digit.append(ch)
            else:
                if current_digit:
                    number = int("".join(current_digit))
                    for x in range(j - len(current_digit), j):
                        data_to_fill[i][x] = (number, part_id)
                    assert part_id not in id_to_no
                    id_to_no[part_id] = number
                    current_digit = []
                    part_id += 1
        if current_digit:
            number = int("".join(current_digit))
            for x in range(j - len(current_digit) + 1, j + 1):
                data_to_fill[i][x] = (number, part_id)
            id_to_no[part_id] = number
            part_id += 1

    seen_ids = set()
    for i in range(len(data)):
        for j in range(len(data[i])):
            if data_to_fill[i][j]:
                continue
            ch = data[i][j]
            if ch == ".":
                continue
            for ii in (-1, 0, 1):
                for jj in (-1, 0, 1):
                    if ii == 0 and jj == 0:
                        continue
                    if i + ii < 0 or i + ii > len(data) - 1:
                        continue
                    if j + jj < 0 or j + jj > len(data[i]) - 1:
                        continue
                    seen_id = data_to_fill[i+ii][j+jj]
                    if seen_id is None:
                        continue
                    seen_id = seen_id[1]
                    if seen_id != 0:
                        seen_ids.add(seen_id)
    return sum(id_to_no[part_id] for part_id in seen_ids)

def part_2(data: list[str]):
    stripped = []
    for d in data:
        stripped.append(d.strip())
    data = stripped
    data_to_fill = []
    for i in range(len(data)):
        curr = []
        for j in range(len(data[i])):
            curr.append(None)
        data_to_fill.append(curr)
    assert len(data) == len(data_to_fill)
    for dd in range(len(data)):
        assert len(data[dd]) == len(data_to_fill[dd])

    id_to_no= {}
    part_id = 1
    for i, line in enumerate(data):
        current_digit = []
        for j, ch in enumerate(line):
            if ch in {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"}:
                current_digit.append(ch)
            else:
                if current_digit:
                    number = int("".join(current_digit))
                    for x in range(j - len(current_digit), j):
                        data_to_fill[i][x] = (number, part_id)
                    assert part_id not in id_to_no
                    id_to_no[part_id] = number
                    current_digit = []
                    part_id += 1
        if current_digit:
            number = int("".join(current_digit))
            for x in range(j - len(current_digit) + 1, j + 1):
                data_to_fill[i][x] = (number, part_id)
            id_to_no[part_id] = number
            part_id += 1

    gear_sum = 0
    for i in range(len(data)):
        for j in range(len(data[i])):
            if data_to_fill[i][j]:
                continue
            ch = data[i][j]
            if ch != "*":
                continue
            seen_ids = set()
            for ii in (-1, 0, 1):
                for jj in (-1, 0, 1):
                    if ii == 0 and jj == 0:
                        continue
                    if i + ii < 0 or i + ii > len(data) - 1:
                        continue
                    if j + jj < 0 or j + jj > len(data[i]) - 1:
                        continue
                    seen_id = data_to_fill[i+ii][j+jj]
                    if seen_id is None:
                        continue
                    seen_id = seen_id[1]
                    seen_ids.add(seen_id)
            if len(seen_ids) == 2:
                seen_ids = list(seen_ids)
                gear_sum += id_to_no[seen_ids[0]] * id_to_no[seen_ids[1]]
    return gear_sum




test_input = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""

print(part_1(test_input.splitlines()))
with open("03_input.txt") as f:
    print(part_1(f.readlines()))

print(part_2(test_input.splitlines()))
with open("03_input.txt") as f:
    print(part_2(f.readlines()))