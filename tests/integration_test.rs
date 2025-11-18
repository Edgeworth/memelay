use memelay::ingest::{load_model, load_seeds};
use memelay::types::{Kc, COLEMAK_DHM, QWERTY};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_load_model_from_real_config() {
    // Test loading an actual config file
    let result = load_model("cfg/layer0.cfg");
    assert!(result.is_ok(), "Failed to load layer0.cfg: {:?}", result.err());

    let model = result.unwrap();
    assert_eq!(model.universe.len(), 30, "Expected 30 keys in universe");
    // Layout includes the 3 rows plus an empty line after
    assert!(model.layout.lines().count() >= 3, "Expected at least 3 rows in layout");
}

#[test]
fn test_load_seeds_from_real_config() {
    // Test loading seed layouts from an actual seed file
    let result = load_seeds("cfg/layer0.seed");
    assert!(result.is_ok(), "Failed to load layer0.seed: {:?}", result.err());

    let seeds = result.unwrap();
    assert!(!seeds.is_empty(), "Expected at least one seed layout");

    // All seeds should have the same size
    let first_size = seeds[0].0.len();
    for seed in &seeds {
        assert_eq!(seed.0.len(), first_size, "All seeds should have same size");
    }
}

#[test]
fn test_qwerty_constant_usage() {
    // Test that QWERTY constant is accessible and valid
    assert_eq!(QWERTY.len(), 30);
    assert_eq!(QWERTY[0], Kc::Q);
    assert_eq!(QWERTY[29], Kc::Slash);
}

#[test]
fn test_colemak_constant_usage() {
    // Test that COLEMAK_DHM constant is accessible and valid
    assert_eq!(COLEMAK_DHM.len(), 30);
    assert_eq!(COLEMAK_DHM[0], Kc::Q);
    assert_eq!(COLEMAK_DHM[2], Kc::F);
}

#[test]
fn test_load_model_with_custom_layout() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "layout").unwrap();
    writeln!(file, "X X X X").unwrap();
    writeln!(file, "X X X X").unwrap();
    writeln!(file, "keys").unwrap();
    writeln!(file, "a b c d").unwrap();
    writeln!(file, "e f g h").unwrap();
    writeln!(file, "fixed").unwrap();
    writeln!(file, "None None None None").unwrap();
    writeln!(file, "None None None None").unwrap();
    writeln!(file, "unigram_cost").unwrap();
    writeln!(file, "1.0 1.0 1.0 1.0").unwrap();
    writeln!(file, "1.0 1.0 1.0 1.0").unwrap();
    writeln!(file, "bigram_cost").unwrap();
    // Need exactly 80 bigram costs
    for _ in 0..80 {
        writeln!(file, "0.0").unwrap();
    }
    writeln!(file, "row").unwrap();
    writeln!(file, "0 0 0 0").unwrap();
    writeln!(file, "1 1 1 1").unwrap();
    writeln!(file, "hand").unwrap();
    writeln!(file, "0 0 1 1").unwrap();
    writeln!(file, "0 0 1 1").unwrap();
    writeln!(file, "finger").unwrap();
    writeln!(file, "0 1 0 1").unwrap();
    writeln!(file, "0 1 0 1").unwrap();
    file.flush().unwrap();

    let result = load_model(file.path());
    assert!(result.is_ok(), "Failed to load custom model: {:?}", result.err());

    let model = result.unwrap();
    assert_eq!(model.universe.len(), 8);
}

#[test]
fn test_load_seeds_with_custom_layout() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a b c d").unwrap();
    writeln!(file, "e f g h").unwrap();
    writeln!(file).unwrap();
    writeln!(file, "h g f e").unwrap();
    writeln!(file, "d c b a").unwrap();
    file.flush().unwrap();

    let result = load_seeds(file.path());
    assert!(result.is_ok(), "Failed to load custom seeds: {:?}", result.err());

    let seeds = result.unwrap();
    assert_eq!(seeds.len(), 2);
    assert_eq!(seeds[0].0.len(), 8);
    assert_eq!(seeds[1].0.len(), 8);
}

#[test]
fn test_model_format_function() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "layout").unwrap();
    writeln!(file, "X X X").unwrap();
    writeln!(file, "keys").unwrap();
    writeln!(file, "a b c").unwrap();
    writeln!(file, "fixed").unwrap();
    writeln!(file, "None None None").unwrap();
    writeln!(file, "unigram_cost").unwrap();
    writeln!(file, "1.0 1.0 1.0").unwrap();
    writeln!(file, "bigram_cost").unwrap();
    for _ in 0..80 {
        writeln!(file, "0.0").unwrap();
    }
    writeln!(file, "row").unwrap();
    writeln!(file, "0 0 0").unwrap();
    writeln!(file, "hand").unwrap();
    writeln!(file, "0 0 1").unwrap();
    writeln!(file, "finger").unwrap();
    writeln!(file, "0 1 2").unwrap();
    file.flush().unwrap();

    let model = load_model(file.path()).unwrap();
    let layout = vec![Kc::A, Kc::B, Kc::C];
    let formatted = model.format(&layout);

    assert!(formatted.contains('a'));
    assert!(formatted.contains('b'));
    assert!(formatted.contains('c'));
}

#[test]
fn test_model_with_fixed_keys() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "layout").unwrap();
    writeln!(file, "X X X X").unwrap();
    writeln!(file, "keys").unwrap();
    writeln!(file, "a b c d").unwrap();
    writeln!(file, "fixed").unwrap();
    writeln!(file, "a None None d").unwrap();
    writeln!(file, "unigram_cost").unwrap();
    writeln!(file, "1.0 1.0 1.0 1.0").unwrap();
    writeln!(file, "bigram_cost").unwrap();
    for _ in 0..80 {
        writeln!(file, "0.0").unwrap();
    }
    writeln!(file, "row").unwrap();
    writeln!(file, "0 0 0 0").unwrap();
    writeln!(file, "hand").unwrap();
    writeln!(file, "0 0 1 1").unwrap();
    writeln!(file, "finger").unwrap();
    writeln!(file, "0 1 0 1").unwrap();
    file.flush().unwrap();

    let model = load_model(file.path()).unwrap();

    // Verify fixed keys are correctly loaded
    assert_eq!(model.fixed[0], Kc::A);
    assert_eq!(model.fixed[1], Kc::None);
    assert_eq!(model.fixed[2], Kc::None);
    assert_eq!(model.fixed[3], Kc::D);

    // Test without_fixed and with_fixed functions
    let test_layout = vec![Kc::A, Kc::B, Kc::C, Kc::D];
    let without = model.without_fixed(&test_layout);
    assert_eq!(without.len(), 2);
    assert_eq!(without[0], Kc::B);
    assert_eq!(without[1], Kc::C);

    let with = model.with_fixed(&without);
    assert_eq!(with.len(), 4);
    assert_eq!(with[0], Kc::A);
    assert_eq!(with[1], Kc::B);
    assert_eq!(with[2], Kc::C);
    assert_eq!(with[3], Kc::D);
}

#[test]
fn test_load_histograms_integration() {
    // Create test histogram files
    let mut uni = NamedTempFile::new().unwrap();
    writeln!(uni, "header").unwrap();
    writeln!(uni, "a 10.0").unwrap();
    writeln!(uni, "b 20.0").unwrap();
    writeln!(uni, "c 30.0").unwrap();
    uni.flush().unwrap();

    let mut bi = NamedTempFile::new().unwrap();
    writeln!(bi, "header").unwrap();
    writeln!(bi, "a b 5.0").unwrap();
    writeln!(bi, "b c 7.5").unwrap();
    bi.flush().unwrap();

    let mut tri = NamedTempFile::new().unwrap();
    writeln!(tri, "header").unwrap();
    writeln!(tri, "a b c 2.5").unwrap();
    tri.flush().unwrap();

    let result = memelay::ingest::load_histograms(
        uni.path(),
        bi.path(),
        tri.path(),
    );

    assert!(result.is_ok(), "Failed to load histograms: {:?}", result.err());

    let hist = result.unwrap();

    // Verify histograms were loaded
    assert!(!hist.unigrams.is_empty());
    assert!(!hist.bigrams.is_empty());
    assert!(!hist.trigrams.is_empty());

    // Verify specific entries exist
    assert!(hist.unigrams.iter().any(|(kc, _)| *kc == Kc::A));
    assert!(hist.unigrams.iter().any(|(kc, _)| *kc == Kc::B));
    assert!(hist.unigrams.iter().any(|(kc, _)| *kc == Kc::C));

    assert!(hist.bigrams.iter().any(|((k1, k2), _)| *k1 == Kc::A && *k2 == Kc::B));
    assert!(hist.bigrams.iter().any(|((k1, k2), _)| *k1 == Kc::B && *k2 == Kc::C));

    assert!(hist.trigrams.iter().any(|((k1, k2, k3), _)| *k1 == Kc::A && *k2 == Kc::B && *k3 == Kc::C));
}

#[test]
fn test_multiple_seed_layouts_different_characteristics() {
    let mut file = NamedTempFile::new().unwrap();

    // First layout: alphabetical
    writeln!(file, "a b c d e f").unwrap();
    writeln!(file).unwrap();

    // Second layout: reverse alphabetical
    writeln!(file, "f e d c b a").unwrap();
    writeln!(file).unwrap();

    // Third layout: mixed
    writeln!(file, "a f b e c d").unwrap();
    file.flush().unwrap();

    let result = load_seeds(file.path());
    assert!(result.is_ok());

    let seeds = result.unwrap();
    assert_eq!(seeds.len(), 3);

    // Verify each layout is different
    assert_ne!(seeds[0].0, seeds[1].0);
    assert_ne!(seeds[1].0, seeds[2].0);
    assert_ne!(seeds[0].0, seeds[2].0);
}

#[test]
fn test_model_cost_arrays_size() {
    let model = load_model("cfg/layer0.cfg").unwrap();

    // Verify unigram cost array has correct size
    assert_eq!(model.unigram_cost.len(), model.universe.len());

    // Bigram cost should have some values
    assert!(!model.bigram_cost.is_empty(), "Bigram cost array should not be empty");
}

#[test]
fn test_model_geometric_properties() {
    let model = load_model("cfg/layer0.cfg").unwrap();

    // Verify geometric arrays match universe size
    assert_eq!(model.row.len(), model.universe.len());
    assert_eq!(model.hand.len(), model.universe.len());
    assert_eq!(model.finger.len(), model.universe.len());

    // Verify rows are in valid range (typically 0-2 for 3 rows)
    for &row in &model.row {
        assert!(row <= 2, "Row value should be 0, 1, or 2");
    }

    // Verify hands are 0 or 1
    for &hand in &model.hand {
        assert!(hand == 0 || hand == 1, "Hand should be 0 (left) or 1 (right)");
    }

    // Verify fingers are in valid range (typically 0-3 for 4 fingers)
    for &finger in &model.finger {
        assert!(finger <= 3, "Finger should be 0-3");
    }
}

#[test]
fn test_empty_layout_handling() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file).unwrap();
    file.flush().unwrap();

    let result = load_seeds(file.path());
    // Should either succeed with empty layout or fail gracefully
    if result.is_ok() {
        let seeds = result.unwrap();
        assert!(seeds.is_empty() || seeds[0].0.is_empty());
    }
}

#[test]
fn test_layout_with_symbols() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, ", . / ; ' -").unwrap();
    file.flush().unwrap();

    let result = load_seeds(file.path());
    assert!(result.is_ok(), "Should handle symbol keys");

    let seeds = result.unwrap();
    assert_eq!(seeds[0].0[0], Kc::Comma);
    assert_eq!(seeds[0].0[1], Kc::Dot);
    assert_eq!(seeds[0].0[2], Kc::Slash);
    assert_eq!(seeds[0].0[3], Kc::Semicolon);
    assert_eq!(seeds[0].0[4], Kc::Quote);
    assert_eq!(seeds[0].0[5], Kc::Minus);
}

#[test]
fn test_kc_string_conversion_roundtrip() {
    let test_chars = vec!['a', 'z', '0', '9', ',', '.', '/', ';'];

    for ch in test_chars {
        let s = ch.to_string();
        let kc = s.parse::<Kc>();
        assert!(kc.is_ok(), "Failed to parse '{}'", ch);

        let kc_val = kc.unwrap();
        assert_eq!(kc_val.to_string(), s, "Roundtrip failed for '{}'", ch);
    }
}

#[test]
fn test_model_universe_contains_all_keys() {
    let model = load_model("cfg/layer0.cfg").unwrap();

    // Universe should contain all the keys from the keys section
    for key in &model.universe {
        assert_ne!(*key, Kc::None, "Universe should not contain None keys");
    }
}
