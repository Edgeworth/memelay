import functools
import itertools
from pathlib import Path
import random

FNAME = "./cfg/bigram_cmp"


def read_cmp():
    cmp = {}
    lines = Path(FNAME).read_text(encoding="utf-8").splitlines()
    for line in lines:
        a1, a2, a3, c, b1, b2, b3 = line.split()
        a = (a1, a2, a3)
        b = (b1, b2, b3)
        val = 0
        if c == "<":
            val = -1
        elif c == ">":
            val = 1
        elif c == "=":
            val = 0
        else:
            assert False
        cmp[(a, b)] = val
    return cmp


fingers = ["index", "middle", "ring", "pinky"]
rows = ["down2", "down1", "same", "up1", "up2"]
precmp = read_cmp()


def compare(a, b):
    if (a, b) in precmp:
        print(f"Using existing value for {(a, b)}: {precmp[(a, b)]}")
        return precmp[(a, b)]
    if (b, a) in precmp:
        print(f"Using existing value for {(a, b)}: {-precmp[(b, a)]}")
        return -precmp[(b, a)]
    a = f"{a[0]} {a[1]} {a[2]}"
    b = f"{b[0]} {b[1]} {b[2]}"
    while True:
        print("Which is easier:")
        print("a:", a)
        print("b:", b)
        ans = input("a / b / 0: ")
        with open(FNAME, "a", encoding="utf-8") as f:
            if ans == "a":
                f.write(f"{a} < {b}\n")
                return -1
            if ans == "b":
                f.write(f"{a} > {b}\n")
                return 1
            if ans == "0":
                f.write(f"{a} = {b}\n")
                return 0
        print("Try again")
    raise Exception("error")


def sort_bigrams():
    bigrams = list(itertools.product(fingers, rows, fingers))
    bigrams.remove(("index", "same", "index"))  # Remove same fingers - hard to compare
    bigrams.remove(("middle", "same", "middle"))
    bigrams.remove(("ring", "same", "ring"))
    bigrams.remove(("pinky", "same", "pinky"))
    random.seed(42)
    random.shuffle(bigrams)
    bigrams.sort(key=functools.cmp_to_key(compare))

    print("\n".join(f"{i[0]} {i[1]} {i[2]}" % i for i in bigrams))


def print_table(filename):
    bigrams = {}
    for same in fingers:
        bigrams[(same, "same", same)] = "0.0"  # default value
    for line in Path(filename).read_text(encoding="utf-8").splitlines():
        a, b, c, v = line.split()
        bigrams[(a, b, c)] = v

    for first in fingers:
        for second in fingers:
            vals = "\t".join(bigrams[(first, row, second)] for row in rows)
            print(vals)
        print()


print_table("cfg/bigram_weights")
