
def write_unigrams(unigrams, unigram_total, suffix):
    unigram_total = float(unigram_total)
    with open('data/unigrams_%s.data' % suffix, 'w') as f:
        f.write('%.18f\n' % unigram_total)
        for k, v in sorted(unigrams.items()):
            f.write('%s %.18f\n' % (k, v / unigram_total))

def write_bigrams(bigrams, bigram_total, suffix):
    bigram_total = float(bigram_total)
    with open('data/bigrams_%s.data' % suffix, 'w') as f:
        f.write('%.18f\n' % bigram_total)
        for k, v in sorted(bigrams.items()):
            f.write('%s %s %.18f\n' % (k[0], k[1], v / bigram_total))
