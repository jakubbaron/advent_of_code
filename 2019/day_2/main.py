# replace position 1 with the value 12 and replace position 2 with the value 2.
def run(values):
    i = 0
    while True:
        op_code= values[i]
        val_1, val_2, output = values[i+1], values[i+2], values[i+3]
        if op_code == 1:
            values[output] = values[val_1] + values[val_2]
        elif op_code == 2:
            values[output] = values[val_1] * values[val_2]
        elif op_code == 99:
            break 
        else:
            break
        i += 4
    return values

def part_1():
    values = [int(x) for x in open('input.txt').read().split(',')]
    values[1] = 12
    values[2] = 2
    run(values)
    print(values[0])

def part_2():
    original_values = [int(x) for x in open('input.txt').read().split(',')]
    for noun in range(100):
        for verb in range(100):
            values = [x for x in original_values]
            values[1] = noun 
            values[2] = verb
            run(values)
            if values[0] == 19690720:
                print(noun, verb, noun*100 + verb)
                return

if __name__ == '__main__':
    part_1()
    part_2()

