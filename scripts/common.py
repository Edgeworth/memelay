def write_unigrams(unigrams, unigram_total, suffix):
    unigram_total = float(unigram_total)
    with open(f"data/unigrams_{suffix}.data", "w", encoding="utf-8") as f:
        f.write(f"{unigram_total:.18f}\n")
        for k, v in sorted(unigrams.items()):
            f.write(f"{k} {v / unigram_total:.18f}\n")


def write_bigrams(bigrams, bigram_total, suffix):
    bigram_total = float(bigram_total)
    with open(f"data/bigrams_{suffix}.data", "w", encoding="utf-8") as f:
        f.write(f"{bigram_total:.18f}\n")
        for k, v in sorted(bigrams.items()):
            f.write(f"{k[0]} {k[1]} {v / bigram_total:.18f}\n")


def write_trigrams(trigrams, trigram_total, suffix):
    trigram_total = float(trigram_total)
    with open(f"data/trigrams_{suffix}.data", "w", encoding="utf-8") as f:
        f.write(f"{trigram_total:.18f}\n")
        for k, v in sorted(trigrams.items()):
            f.write(f"{k[0]} {k[1]} {k[2]} {v / trigram_total:.18f}\n")
