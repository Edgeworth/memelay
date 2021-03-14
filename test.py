import os
import string

f = open('data/data_time.data')
prev_t = 0
THRESH = 1.0
m = {}
for l in f.readlines():
    t, key, pressed = l.split(' ')
    t = int(t) / 1000000.0
    if pressed == '0':
        continue
    d = t - prev_t
    if d >= 0 and d < THRESH:
        m.setdefault(key, [0.0, 0])
        m[key][0] += d
        m[key][1] += 1
    prev_t = t

for k, v in m.items():
    if k in string.ascii_letters:
        print(k, 1000.0 * (v[0] / v[1]))
