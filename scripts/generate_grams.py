from pathlib import Path
import string

from scripts.common import write_bigrams
from scripts.common import write_trigrams
from scripts.common import write_unigrams

filelist = "dropbox"
layer = "layer1"

trans = str.maketrans(":<>?", ";,./", "")
files = [
    i.strip() for i in Path("data/filelist_" + filelist).read_text(encoding="utf-8").splitlines()
]

allow_map = {
    "layer0": string.ascii_lowercase + ";,./",
    "layer1": "|*{}\"+_789#!()'=-456@&[]$\\0123",
}
allowed = allow_map[layer]

unigrams: dict[str, int] = {}
bigrams: dict[tuple[str, str], int] = {}
trigrams: dict[tuple[str, str, str], int] = {}
for file in files:
    try:
        data = Path(file).read_text(encoding="utf-8").lower()
    except Exception:
        print("Error processing, skipping ", file)
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

suffix = f"{filelist}_{layer}"
unigram_total = sum(i for i in unigrams.values())
bigram_total = sum(i for i in bigrams.values())
trigram_total = sum(i for i in trigrams.values())
write_unigrams(unigrams, unigram_total, suffix)
write_bigrams(bigrams, bigram_total, suffix)
write_trigrams(trigrams, trigram_total, suffix)
