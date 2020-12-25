import time

class Node:
    def __init__(self, val: int):
        self.val = val
        self.next = None

    def print(self, game_len: int):
        node = self
        i = 0
        output = ""
        while node is not None and i < game_len:
            output += str(node.val) + " "
            node = node.next
            i += 1
        print(output)
    def __repr__(self):
        return f"Val: {self.val} Next: {self.next is None}"

    def result_1(self, game_len):
        current = self
        while current.val != 1:
            current = current.next
        output = ""
        current = current.next
        for _ in range(game_len-1):
            output += str(current.val)
            current = current.next
        return output

    def result_2(self, game_len):
        current = self
        while current.val != 1:
            current = current.next
        val_1 = current.next.val
        val_2 = current.next.next.val
        return val_1 * val_2

    def play_game(self, iterations, game_map, game_len, max_cup):
        current = self
        # self.print(game_len)
        start_time = time.time()
        for i in range(iterations):
            # if i % 1_000 == 0:
            #     current_time = time.time()
            #     print(f"Current iter {i}, {current_time-start_time}")
            #     start_time = current_time
            current_value = current.val
            first_of_three = current.next
            third_in_front = current
            # print(f"First of three {first_of_three.val}")
            for _ in range(3):
                third_in_front = third_in_front.next
            # print(f"Third in front {third_in_front.val}")

            sought_value = current_value - 1
            if sought_value == 0:
                sought_value = max_cup
            all_good = False
            while not all_good:
                all_good = True
                tmp = first_of_three
                for i in range(3):
                    if tmp.val == sought_value:
                        sought_value -= 1
                        if sought_value == 0:
                            sought_value = max_cup
                        all_good = False
                        break
                    tmp = tmp.next
            new_tail = game_map[sought_value]

            temp_old_tail = new_tail.next
            current.next = third_in_front.next
            new_tail.next = first_of_three
            third_in_front.next = temp_old_tail
            current = current.next
            # current.print(game_len)
        return current



def run_code(input_vals, result_1, result_2):
    node = Node(int(input_vals[0]))
    max_cup = max(int(x) for x in input_vals)
    head = node
    for val in input_vals[1:]:
        node.next = Node(int(val))
        node = node.next
    node.next = head
    game_len = len(input_vals)
    head.print(game_len)
    tmp = head
    game_map = {}
    while True:
        val = tmp.val
        game_map[val] = tmp
        tmp = tmp.next
        if tmp == head:
            break

    current = head.play_game(100, game_map, game_len, max_cup)
    current.print(game_len)
    assert current.result_1(game_len) == result_1

    values = list(range(1, 1_000_001))
    for i, val in enumerate(input_vals):
        values[i] = int(val)
    node = Node(values[0])
    max_cup = 1_000_000
    head = node
    for val in values[1:]:
        node.next = Node(val)
        node = node.next
    node.next = head
    game_len = len(values)
    tmp = head
    game_map = {}
    while True:
        val = tmp.val
        game_map[val] = tmp
        tmp = tmp.next
        if tmp == head:
            break
    current = head.play_game(10_000_000, game_map, game_len, max_cup)
    assert current.result_2(game_len) == result_2

if __name__ == "__main__":
    run_code("389125467", "67384529", 149245887792)
    run_code("315679824", "72496583", 41785843847)

