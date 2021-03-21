import os
import string

# Skip whitespace without breaking bigrams - typed by thumb keys
skip = str.maketrans(':<>?', ';,./', ' \n\t\r')
files = [i.strip() for i in open('data/filelist').readlines()]
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

with open('data/unigrams.data', 'w') as f:
    for k, v in sorted(unigrams.items()):
        f.write('%s %d\n'% (k, v))
with open('data/bigrams.data', 'w') as f:
    for k, v in sorted(bigrams.items()):
        f.write('%s %s %d\n' % (k[0], k[1], v))
