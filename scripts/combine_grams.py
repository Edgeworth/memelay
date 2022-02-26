from pathlib import Path
import sys

from scripts.common import write_bigrams
from scripts.common import write_trigrams
from scripts.common import write_unigrams

layer = sys.argv[1]
suffix1 = sys.argv[2]
suffix2 = sys.argv[3]


def read_unigrams(name):
    lines = Path(name).read_text(encoding="utf-8").splitlines()[1:]
    unigrams = {}
    for line in lines:
        ch, val = line.split(" ")
        unigrams[ch] = float(val)
    return unigrams


def read_bigrams(name):
    lines = Path(name).read_text(encoding="utf-8").splitlines()[1:]
    bigrams = {}
    for line in lines:
        ch1, ch2, val = line.split(" ")
        bigrams[(ch1, ch2)] = float(val)
    return bigrams


def read_trigrams(name):
    lines = Path(name).read_text(encoding="utf-8").splitlines()[1:]
    trigrams = {}
    for line in lines:
        ch1, ch2, ch3, val = line.split(" ")
        trigrams[(ch1, ch2, ch3)] = float(val)
    return trigrams


def combine_dist(a, b):
    keys = set(a.keys())
    keys = keys.union(set(b.keys()))
    output = {}
    for k in keys:
        output[k] = a.get(k, 0.0) * 0.5 + b.get(k, 0.0) * 0.5
    return output


p1uni = read_unigrams(f"data/unigrams_{suffix1}_{layer}.data")
p2uni = read_unigrams(f"data/unigrams_{suffix2}_{layer}.data")
p1bi = read_bigrams(f"data/bigrams_{suffix1}_{layer}.data")
p2bi = read_bigrams(f"data/bigrams_{suffix2}_{layer}.data")
p1tri = read_trigrams(f"data/trigrams_{suffix1}_{layer}.data")
p2tri = read_trigrams(f"data/trigrams_{suffix2}_{layer}.data")


out_suffix = f"combined_{layer}"
write_unigrams(combine_dist(p1uni, p2uni), 1.0, out_suffix)
write_bigrams(combine_dist(p1bi, p2bi), 1.0, out_suffix)
write_trigrams(combine_dist(p1tri, p2tri), 1.0, out_suffix)
