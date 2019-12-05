# replace position 1 with the value 12 and replace position 2 with the value 2.
def run(values, input_parameter):
    i = 0

    def get_value(values, value, mode):
        if mode == '1':
            return value
        else:
            return values[value]

    while True:
        parameter_mode = values[i]
        A, B, C, D, E = str(parameter_mode).zfill(5)
        op_code = int(D + E)
        if op_code == 1:
            val_1 = get_value(values, values[i + 1], C)
            val_2 = get_value(values, values[i + 2], B)
            # Parameters that an instruction writes to will never be in immediate mode.
            values[values[i + 3]] = val_1 + val_2
            i += 4
        elif op_code == 2:
            val_1 = get_value(values, values[i + 1], C)
            val_2 = get_value(values, values[i + 2], B)
            # Parameters that an instruction writes to will never be in immediate mode.
            values[values[i + 3]] = val_1 * val_2
            i += 4
        elif op_code == 3:
            values[values[i + 1]] = input_parameter
            i += 2
        elif op_code == 4:
            val_1 = get_value(values, values[i + 1], C)
            print("Output:", val_1)
            i += 2
        elif op_code == 5:
            val_1 = get_value(values, values[i + 1], C)
            val_2 = get_value(values, values[i + 2], B)
            if val_1 != 0:
                i = val_2
            else:
                i += 3
        elif op_code == 6:
            val_1 = get_value(values, values[i + 1], C)
            val_2 = get_value(values, values[i + 2], B)
            if val_1 == 0:
                i = val_2
            else:
                i += 3
        elif op_code == 7:
            val_1 = get_value(values, values[i + 1], C)
            val_2 = get_value(values, values[i + 2], B)
            values[values[i + 3]] = int(val_1 < val_2)
            i += 4
        elif op_code == 8:
            val_1 = get_value(values, values[i + 1], C)
            val_2 = get_value(values, values[i + 2], B)
            values[values[i + 3]] = int(val_1 == val_2)
            i += 4
        elif op_code == 99:
            break
        else:
            raise ValueError()

    return values


def part_1():
    values = [int(x) for x in open('input.txt').read().split(',')]
    run(values, 1)


def part_2():
    values = [int(x) for x in open('input.txt').read().split(',')]
    run(values, 5)


if __name__ == '__main__':
    part_1()
    part_2()
