import json

# Converts kbd output to
# https://stevep99.github.io/keyboard-layout-analyzer/#/config format.

src = """
z r d f v / u , . q
x n s t p y e a o g
w m c l b k i ; h j
""".strip().lower().split()

shift_map = {
    96: 126,
    49: 33,
    50: 64,
    51: 35,
    52: 36,
    53: 37,
    54: 94,
    55: 38,
    56: 42,
    57: 40,
    48: 41,
    91: 123,
    93: 125,
    39: 34,
    44: 60,
    46: 62,
    112: 80,
    121: 89,
    102: 70,
    103: 71,
    99: 67,
    114: 82,
    108: 76,
    47: 63,
    61: 43,
    92: 124,
    97: 65,
    111: 79,
    101: 69,
    117: 85,
    105: 73,
    100: 68,
    104: 72,
    116: 84,
    110: 78,
    115: 83,
    45: 95,
    59: 58,
    113: 81,
    106: 74,
    107: 75,
    120: 88,
    98: 66,
    109: 77,
    119: 87,
    118: 86,
    122: 90,
}

template = json.loads(open('data/keyboard_layout_analyzer.json').read())

idx = 0
for key in template["keys"]:
    if key["primary"] == 0:
        key["primary"] = ord(src[idx])
        key["shift"] = shift_map[ord(src[idx])]
        idx += 1

assert idx == 30

print(json.dumps(template))
