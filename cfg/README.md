## Costs
Consider 1.0 to be about the cost of pressing a home row key with your middle
finger.
See unigram cost table.

### unigram_cost:
https://stevep99.github.io/keyboard-effort-grid/ with modifications
Pinkies made to a high cost.

8.0	2.4	2.0	2.2	3.2	3.2	2.2	2.0	2.4	8.0
4.0	1.3	1.1	1.0	2.9	2.9	1.0	1.1	1.3	4.0
7.5	2.6	2.3	1.6	3.0	3.0	1.6	2.3	2.6	7.5

### bigram_cost:
e.g. read into this table:
```rust
[
    // First finger: index - second finger: [down 2, down 1, same row, up 1, up 2]
    [0.0, 0.0, 2.5, 3.0, 4.0], // Index - same row val only used for different key locations
    [0.0, 0.0, 0.5, 1.0, 2.0], // Middle - outward roll
    [0.0, 0.0, 0.5, 0.8, 1.5], // Ring - outward roll
    [0.0, 0.0, 0.5, 0.7, 1.1], // Pinkie - outward roll
],
[
    // First finger: middle - second finger: [down 2, down 1, same row, up 1, up 2]
    [0.0, 0.0, -1.5, -0.5, 1.5], // Index - inward roll
    [0.0, 0.0, 0.0, 3.5, 4.5],   // Middle - same row val only used for different key locations
    [0.0, 0.0, 0.5, 1.0, 2.0],   // Ring - outward roll
    [0.0, 0.0, 0.5, 0.8, 1.5],   // Pinkie - outward roll
],
[
    // First finger: ring - second finger: [down 2, down 1, same row, up 1, up 2]
    [0.0, 0.0, -1.5, -0.5, 1.5], // Index - inward roll
    [0.0, 0.0, -2.0, -0.5, 1.2], // Middle - inward roll
    [0.0, 0.0, 0.0, 3.5, 4.5],   // Ring - same row val only used for different key locations
    [0.0, 0.0, 0.0, 3.5, 4.5],   // Pinkie - outward roll
],
[
    // First finger: pinkie - second finger: [down 2, down 1, same row, up 1, up 2]
    [0.0, 0.0, -1.0, 0.0, 1.0], // Index - inward roll
    [0.0, 0.0, -1.0, 0.0, 1.5], // Middle - inward roll
    [0.0, 0.0, -1.0, 0.0, 1.5], // Ring - inward roll
    [0.0, 0.0, 3.0, 4.0, 5.5],  // Pinkie - same row val only used for different key locations
],
```

## Layout

### hand:
0 = left, 1 = right

### finger:
3 = pinkie, 2 = ring, 1 = middle, 0 = index
