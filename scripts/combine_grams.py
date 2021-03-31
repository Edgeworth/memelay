import sys
from scripts.common import write_unigrams, write_bigrams, write_trigrams

layer = sys.argv[1]
suffix1 = sys.argv[2]
suffix2 = sys.argv[3]

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

def read_trigrams(name):
    lines = open(name).readlines()[1:]
    trigrams = {}
    for l in lines:
        ch1, ch2, ch3, val = l.split(' ')
        trigrams[(ch1, ch2, ch3)] = float(val)
    return trigrams


def combine_dist(a, b):
    keys = set(a.keys())
    keys = keys.union(set(b.keys()))
    output = {}
    for k in keys:
        output[k] = a.get(k, 0.0) * 0.5 + b.get(k, 0.0) * 0.5
    return output

p1uni = read_unigrams('data/unigrams_%s_%s.data' % (suffix1, layer))
p2uni = read_unigrams('data/unigrams_%s_%s.data' % (suffix2, layer))
p1bi = read_bigrams('data/bigrams_%s_%s.data' % (suffix1, layer))
p2bi = read_bigrams('data/bigrams_%s_%s.data' % (suffix2, layer))
p1tri = read_trigrams('data/trigrams_%s_%s.data' % (suffix1, layer))
p2tri = read_trigrams('data/bigrams_%s_%s.data' % (suffix2, layer))


out_suffix = 'combined_%s' % layer
write_unigrams(combine_dist(p1uni, p2uni), 1.0, out_suffix)
write_bigrams(combine_dist(p1bi, p2bi), 1.0, out_suffix)
write_trigrams(combine_dist(p1tri, p2tri), 1.0, out_suffix)
