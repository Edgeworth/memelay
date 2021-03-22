import os
import string

# Skip whitespace without breaking bigrams - typed by thumb keys
skip = str.maketrans(':<>?', ';,./', ' \n\t\r')
suffix = '_linux'
files = [i.strip() for i in open('data/filelist' + suffix).readlines()]
allowed = string.ascii_lowercase + ';,./'

unigrams = {}
bigrams = {}
for file in files:
    try:
        data = open(file).read().lower()
    except:
        print('Error processing, skipping ', file)
        continue
    data = data.translate(skip)

    prev = None
    for c in data:
        if c not in allowed:
            prev = None  # break bigrams
            continue

        unigrams.setdefault(c, 0)
        unigrams[c] += 1

        if prev is not None:
            bgram = (prev, c)
            bigrams.setdefault(bgram, 0)
            bigrams[bgram] += 1
        prev = c

unigram_total = sum(i for i in unigrams.values())
bigram_total = sum(i for i in bigrams.values())

with open('data/unigrams%s.data' % suffix, 'w') as f:
    f.write('%d\n' % unigram_total)
    for k, v in sorted(unigrams.items()):
        f.write('%s %.18f\n' % (k, v / unigram_total))
with open('data/bigrams%s.data' % suffix, 'w') as f:
    f.write('%d\n' % bigram_total)
    for k, v in sorted(bigrams.items()):
        f.write('%s %s %.18f\n' % (k[0], k[1], v / bigram_total))
