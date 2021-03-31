import itertools
import functools
import random

FNAME = './cfg/bigram_cmp'

def read_cmp():
    cmp = {}
    lines = open(FNAME).readlines()
    for line in lines:
        a1, a2, a3, c, b1, b2, b3 = line.split()
        a = (a1, a2, a3)
        b = (b1, b2, b3)
        val = 0
        if c == '<':
            val = -1
        elif c == '>':
            val = 1
        elif c == '=':
            val = 0
        else:
            assert False
        cmp[(a, b)] = val
    return cmp


fingers = ['index', 'middle', 'ring', 'pinky']
rows = ['down2', 'down1', 'same', 'up1', 'up2']
precmp = read_cmp()

def compare(a, b):
    if (a, b) in precmp:
        print('Using existing value for %s: %d' % (str((a, b)), precmp[(a, b)]))
        return precmp[(a, b)]
    if (b, a) in precmp:
        print('Using existing value for %s: %d' % (str((a, b)), -precmp[(b, a)]))
        return -precmp[(b, a)]
    a = '%s %s %s' % a
    b = '%s %s %s' % b
    while True:
        print('Which is easier:')
        print('a:', a)
        print('b:', b)
        ans = input('a / b / 0: ')
        with open(FNAME, 'a') as f:
            if ans == 'a':
                f.write('%s < %s\n' % (a, b))
                return -1
            elif ans == 'b':
                f.write('%s > %s\n' % (a, b))
                return 1
            elif ans == '0':
                f.write('%s = %s\n' % (a, b))
                return 0
        print("Try again")
    raise "error"

def sort_bigrams():
    bigrams = list(itertools.product(fingers, rows, fingers))
    bigrams.remove(('index', 'same', 'index'))  # Remove same fingers - hard to compare
    bigrams.remove(('middle', 'same', 'middle'))
    bigrams.remove(('ring', 'same', 'ring'))
    bigrams.remove(('pinky', 'same', 'pinky'))
    random.seed(42)
    random.shuffle(bigrams)
    bigrams.sort(key=functools.cmp_to_key(compare))

    print('\n'.join('%s %s %s' % i for i in bigrams))


def print_table(filename):
    bigrams = {}
    for same in fingers:
        bigrams[(same, 'same', same)] = '0.0'  # default value
    for line in open(filename).readlines():
        a, b, c, v = line.split()
        bigrams[(a, b, c)] = v


    for first in fingers:
        for second in fingers:
            vals = '\t'.join(bigrams[(first, row, second)] for row in rows)
            print(vals)
        print()

print_table('cfg/bigram_weights')
