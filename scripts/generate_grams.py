import os
import string
from typing import Dict, Tuple
from scripts.common import write_unigrams, write_bigrams, write_trigrams

filelist = 'gutenberg'
layer = 'layer0'

trans = str.maketrans(':<>?', ';,./', '')
files = [i.strip() for i in open('data/filelist_' + filelist).readlines()]

allow_map = {
    'layer0': string.ascii_lowercase + ';,./',
    'layer1': '|*{}"+_789#!()\'=-456@&[]$\\0123'
}
allowed = allow_map[layer]

unigrams: Dict[str, int] = {}
bigrams: Dict[Tuple[str, str], int] = {}
trigrams: Dict[Tuple[str, str, str], int] = {}
for file in files:
    try:
        data = open(file).read().lower()
    except:
        print('Error processing, skipping ', file)
        continue
    data = data.translate(trans)

    prev = None
    pprev = None
    for c in data:
        if c not in allowed:
            prev = None  # break bigrams
            pprev = None
            continue

        unigrams.setdefault(c, 0)
        unigrams[c] += 1

        if prev is not None:
            bgram = (prev, c)
            bigrams.setdefault(bgram, 0)
            bigrams[bgram] += 1

            if pprev is not None:
                tgram = (pprev, prev, c)
                trigrams.setdefault(tgram, 0)
                trigrams[tgram] += 1
        pprev = prev
        prev = c

suffix = '%s_%s' % (filelist, layer)
unigram_total = sum(i for i in unigrams.values())
bigram_total = sum(i for i in bigrams.values())
trigram_total = sum(i for i in trigrams.values())
write_unigrams(unigrams, unigram_total, suffix)
write_bigrams(bigrams, bigram_total, suffix)
write_trigrams(trigrams, trigram_total, suffix)
