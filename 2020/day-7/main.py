from typing import List

def bags(lines: List[str]) -> int:
    return -1

if __name__ == "__main__":
    with open("test.txt", "r") as f:
        lines = f.readlines()
        print("Bags", bags(lines))
