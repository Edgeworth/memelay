// Runtime (typing time) limits:
// Maximum number of physical keys to allow pressing a time.
pub const MAX_PRESS: usize = 4;
// Maximum number of regular keycodes to allow pressing at a time.
pub const MAX_KEY_REG_ASSIGN: usize = 1;
// Maximum number of mod keycodes to allow pressing at a time.
pub const MAX_KEY_MOD_ASSIGN: usize = 4;
// Maximum number of physical key-strokes without generating any keycodes.
pub const MAX_IDLE: usize = 4;
// Maximum count of each mod in kcset.
pub const MAX_KCSET_MOD: usize = 1;

// Layout limits:
// Maximum number of keys with mods per layer.
pub const MAX_MODS_PER_LAYER: usize = 10;
// Maximum of two of the same physical keys.
pub const MAX_SAME: usize = 2;

// Fitness config:
// Maximum batch size
pub const BATCH_SIZE: usize = 100;
// How many batches to run per fitness solve.
pub const NUM_BATCH: usize = 100;
