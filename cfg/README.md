unigram_cost:
https://stevep99.github.io/keyboard-effort-grid/ with modifications
Pinkies made to a high cost.

bigram_cost:
e.g. read into this table:
```rust
[
    // First finger: index
    [2.5, 3.0, 4.0], // Index - same row val only used for different key locations
    [0.5, 1.0, 2.0], // Middle - outward roll
    [0.5, 0.8, 1.5], // Ring - outward roll
    [0.5, 0.7, 1.1], // Pinkie - outward roll
],
[
    // First finger: middle
    [-1.5, -0.5, 1.5], // Index - inward roll
    [0.0, 3.5, 4.5],   // Middle - same row val only used for different key locations
    [0.5, 1.0, 2.0],   // Ring - outward roll
    [0.5, 0.8, 1.5],   // Pinkie - outward roll
],
[
    // First finger: ring
    [-1.5, -0.5, 1.5], // Index - inward roll
    [-2.0, -0.5, 1.2], // Middle - inward roll
    [0.0, 3.5, 4.5],   // Ring - same row val only used for different key locations
    [0.0, 3.5, 4.5],   // Pinkie - outward roll
],
[
    // First finger: pinkie
    [-1.0, 0.0, 1.0], // Index - inward roll
    [-1.0, 0.0, 1.5], // Middle - inward roll
    [-1.0, 0.0, 1.5], // Ring - inward roll
    [3.0, 4.0, 5.5],  // Pinkie - same row val only used for different key locations
],
```

hand:
0 = left, 1 = right

finger:
3 = pinkie, 2 = ring, 1 = middle, 0 = index
