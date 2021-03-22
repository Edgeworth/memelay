#!/usr/bin/python
import os
import string

f = open('data/keys_time.data')

SHFT_MAP = {
    'A': 'A',
    'B': 'B',
    'C': 'C',
    'D': 'D',
    'E': 'E',
    'F': 'F',
    'G': 'G',
    'H': 'H',
    'I': 'I',
    'J': 'J',
    'K': 'K',
    'L': 'L',
    'M': 'M',
    'N': 'N',
    'O': 'O',
    'P': 'P',
    'Q': 'Q',
    'R': 'R',
    'S': 'S',
    'T': 'T',
    'U': 'U',
    'V': 'V',
    'W': 'W',
    'X': 'X',
    'Y': 'Y',
    'Z': 'Z',
    '0': ')',
    '1': '!',
    '2': '@',
    '3': '#',
    '4': '$',
    '5': '%',
    '6': '^',
    '7': '&',
    '8': '*',
    '9': '(',
    'F0': 'F0',
    'F1': 'F1',
    'F2': 'F2',
    'F3': 'F3',
    'F4': 'F4',
    'F5': 'F5',
    'F6': 'F6',
    'F7': 'F7',
    'F8': 'F8',
    'F9': 'F9',
    'F10': 'F10',
    'F11': 'F11',
    'F12': 'F12',
    'SPACE': 'SPACE',
    'TAB': 'TAB',
    'ENTER': 'ENTER',
    'LCTRL': 'LCTRL',
    'LALT': 'LALT',
    'LSHIFT': 'LSHIFT',
    'BACKSPACE': 'BACKSPACE',
    'EQUAL': 'PLUS',
    'SEMICOLON': 'COLON',
    'DOT': 'GT',
    'COMMA': 'LT',
    'LARROW': 'LARROW',
    'RARROW': 'RARROW',
    'UARROW': 'UARROW',
    'DARROW': 'DARROW',
    'PGDN': 'PGDN',
    'PGUP': 'PGUP',
    'RALT': 'RALT',
    'RSUPER': 'RSUPER',
    'INS': 'INS',
    'MENU': 'MENU',
    'PABR': 'PABR',
    'RCTRL': 'RCTRL',
    'APOSTROPHE': '"',
    'GRAVE': '~',
    'BACKSLASH': '|',
    'LBRACE': '{',
    'RBRACE': '}',
    'ESC': 'ESC',
    'END': 'END',
    'DEL': 'DEL',
    'LSUPER': 'LSUPER',
    'RSHIFT': 'RSHIFT',
    'PSSR': 'PSSR',
    'SCROLLLOCK': 'SCROLLLOCK',
    'HOME': 'HOME',
    'SLASH': 'QMARK',
    'MINUS': 'UNDERSCORE',
}

def press_times():
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
        print(k, 1000.0 * (v[0] / v[1]))

def histogram():
    m = {}
    shft = False
    for l in f.readlines():
        _, key, pressed = l.split(' ')
        if key == 'LSHIFT':
            shft = int(pressed.strip()) != 0
        if pressed == '0':
            continue

        if shft:
            key = SHFT_MAP[key]
        m.setdefault(key, 0)
        m[key] += 1
    s = sorted([(v, k) for k, v in m.items()], reverse=True)
    for count, key in s:
        print(count, key)

def histogram_not_shifted():
    m = {}
    for l in f.readlines():
        _, key, pressed = l.split(' ')
        if pressed == '0':
            continue

        m.setdefault(key, 0)
        m[key] += 1
    s = sorted([(v, k) for k, v in m.items()], reverse=True)
    for count, key in s:
        print(count, key)

def clean():
    for l in f.readlines():
        _, key, pressed = l.split(' ')
        if pressed.strip() == '0':
            continue
        key = key.lower()
        if key in string.ascii_lowercase:
            print(key)
        if key == 'dot':
            print('.')
        if key == 'semicolon':
            print(';')
        if key == 'comma':
            print(',')
        if key == 'slash':
            print('/')
clean()
