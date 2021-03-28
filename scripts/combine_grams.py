import sys
from scripts.common import write_unigrams, write_bigrams

suffix1 = sys.argv[1]
suffix2 = sys.argv[2]

def read_unigrams(name):
    lines = open(name).readlines()[1:]
    unigrams = {}
    for l in lines:
        ch, val = l.split(' ')
        unigrams[ch] = float(val)
    return unigrams

def read_bigrams(name):
    lines = open(name).readlines()[1:]
    bigrams = {}
    for l in lines:
        ch1, ch2, val = l.split(' ')
        bigrams[(ch1, ch2)] = float(val)
    return bigrams

def combine_dist(a, b):
    keys = set(a.keys())
    keys = keys.union(set(b.keys()))
    output = {}
    for k in keys:
        output[k] = a.get(k, 0.0) * 0.5 + b.get(k, 0.0) * 0.5
    return output

p1uni = read_unigrams('data/unigrams_%s.data' % suffix1)
p2uni = read_unigrams('data/unigrams_%s.data' % suffix2)
p1bi = read_bigrams('data/bigrams_%s.data' % suffix1)
p2bi = read_bigrams('data/bigrams_%s.data' % suffix2)

write_unigrams(combine_dist(p1uni, p2uni), 1.0, 'combined')
write_bigrams(combine_dist(p1bi, p2bi), 1.0, 'combined')
