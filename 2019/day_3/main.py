import pprint
pp = pprint.PrettyPrinter(indent=4)
def traverse_wire(wire):
    DX = {'L': -1, 'R': 1, 'U': 0, 'D': 0}
    DY = {'L': 0, 'R': 0, 'U': 1, 'D': -1}
    x = 0
    y = 0
    length = 0
    ans = {}
    for move in wire:
        d = move[0]
        l = int(move[1:])
        assert d in ('L','R','U','D')
        for _ in range(l):
            x += DX[d]
            y += DY[d]
            length += 1
            if (x,y) not in ans:
                ans[(x,y)] = length
    return ans

def part_1(wires):
    wire_1, wire_2 = wires
    W1 = traverse_wire(wire_1) 
    W2 = traverse_wire(wire_2) 
    both = set(W1.keys()) & set(W2.keys())
    return min((abs(x)+abs(y) for (x,y) in both))

def part_2(wires):
    wire_1, wire_2 = wires
    W1 = traverse_wire(wire_1) 
    W2 = traverse_wire(wire_2) 
    both = set(W1.keys()) & set(W2.keys())
    return min((W1[p]+W2[p] for p in both))

if __name__ == '__main__':
    f = open('input.txt')
    wires = f.readlines()
    wire_1 = wires[0].split(',')
    wire_2 = wires[1].split(',')
    f.close()
    wires = (wire_1, wire_2)
    print(part_1(wires))
    print(part_2(wires))
