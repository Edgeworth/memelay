use memelay::eval::KeyState;
use memelay::ingest::{load_model, load_seeds};
use memelay::types::Kc;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_complete_test_environment() -> (NamedTempFile, NamedTempFile, NamedTempFile, NamedTempFile, NamedTempFile) {
    // Model file
    let mut model = NamedTempFile::new().unwrap();
    writeln!(model, "layout").unwrap();
    writeln!(model, "X X X X").unwrap();
    writeln!(model, "keys").unwrap();
    writeln!(model, "a b c d").unwrap();
    writeln!(model, "fixed").unwrap();
    writeln!(model, "None None None None").unwrap();
    writeln!(model, "unigram_cost").unwrap();
    writeln!(model, "1.0 1.5 2.0 2.5").unwrap();
    writeln!(model, "bigram_cost").unwrap();
    for _ in 0..80 {
        writeln!(model, "1.0").unwrap();
    }
    writeln!(model, "row").unwrap();
    writeln!(model, "0 0 1 1").unwrap();
    writeln!(model, "hand").unwrap();
    writeln!(model, "0 0 1 1").unwrap();
    writeln!(model, "finger").unwrap();
    writeln!(model, "0 1 0 1").unwrap();
    model.flush().unwrap();

    // Seed file
    let mut seeds = NamedTempFile::new().unwrap();
    writeln!(seeds, "a b c d").unwrap();
    writeln!(seeds).unwrap();
    writeln!(seeds, "b a d c").unwrap();
    seeds.flush().unwrap();

    // Histogram files
    let mut uni = NamedTempFile::new().unwrap();
    writeln!(uni, "header").unwrap();
    writeln!(uni, "a 10.0").unwrap();
    writeln!(uni, "b 8.0").unwrap();
    writeln!(uni, "c 6.0").unwrap();
    writeln!(uni, "d 4.0").unwrap();
    uni.flush().unwrap();

    let mut bi = NamedTempFile::new().unwrap();
    writeln!(bi, "header").unwrap();
    writeln!(bi, "a b 5.0").unwrap();
    writeln!(bi, "b c 3.0").unwrap();
    bi.flush().unwrap();

    let mut tri = NamedTempFile::new().unwrap();
    writeln!(tri, "header").unwrap();
    writeln!(tri, "a b c 1.0").unwrap();
    tri.flush().unwrap();

    (model, seeds, uni, bi, tri)
}

#[test]
fn test_load_and_format_workflow() {
    let (model_file, _seeds, _uni, _bi, _tri) = create_complete_test_environment();

    let model = load_model(model_file.path()).unwrap();
    let layout = vec![Kc::A, Kc::B, Kc::C, Kc::D];

    let formatted = model.format(&layout);

    // Verify formatting works
    assert!(formatted.contains('a'));
    assert!(formatted.contains('b'));
    assert!(formatted.contains('c'));
    assert!(formatted.contains('d'));
}

#[test]
fn test_load_seeds_and_validate() {
    let (model_file, seed_file, _uni, _bi, _tri) = create_complete_test_environment();

    let model = load_model(model_file.path()).unwrap();
    let seeds = load_seeds(seed_file.path()).unwrap();

    // Verify seeds were loaded
    assert_eq!(seeds.len(), 2);

    // Verify each seed has the correct size
    for seed in &seeds {
        assert_eq!(seed.0.len(), model.universe.len());
    }
}

#[test]
fn test_fixed_keys_workflow() {
    let mut model_file = NamedTempFile::new().unwrap();
    writeln!(model_file, "layout").unwrap();
    writeln!(model_file, "X X X X X X").unwrap();
    writeln!(model_file, "keys").unwrap();
    writeln!(model_file, "a b c d e f").unwrap();
    writeln!(model_file, "fixed").unwrap();
    writeln!(model_file, "a None None None None f").unwrap();
    writeln!(model_file, "unigram_cost").unwrap();
    writeln!(model_file, "1.0 1.5 2.0 2.5 3.0 3.5").unwrap();
    writeln!(model_file, "bigram_cost").unwrap();
    for _ in 0..80 {
        writeln!(model_file, "1.0").unwrap();
    }
    writeln!(model_file, "row").unwrap();
    writeln!(model_file, "0 0 1 1 2 2").unwrap();
    writeln!(model_file, "hand").unwrap();
    writeln!(model_file, "0 0 0 1 1 1").unwrap();
    writeln!(model_file, "finger").unwrap();
    writeln!(model_file, "0 1 0 1 0 1").unwrap();
    model_file.flush().unwrap();

    let model = load_model(model_file.path()).unwrap();

    // Test the fixed keys workflow
    let full_layout = vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F];

    // Extract non-fixed keys
    let non_fixed = model.without_fixed(&full_layout);
    assert_eq!(non_fixed.len(), 4);
    assert_eq!(non_fixed, vec![Kc::B, Kc::C, Kc::D, Kc::E]);

    // Reconstruct with fixed keys
    let reconstructed = model.with_fixed(&non_fixed);
    assert_eq!(reconstructed.len(), 6);
    assert_eq!(reconstructed[0], Kc::A);
    assert_eq!(reconstructed[5], Kc::F);
    assert_eq!(reconstructed, full_layout);
}

#[test]
fn test_model_without_fixed_preserves_order() {
    let mut model_file = NamedTempFile::new().unwrap();
    writeln!(model_file, "layout").unwrap();
    writeln!(model_file, "X X X X X").unwrap();
    writeln!(model_file, "keys").unwrap();
    writeln!(model_file, "a b c d e").unwrap();
    writeln!(model_file, "fixed").unwrap();
    writeln!(model_file, "None b None d None").unwrap();
    writeln!(model_file, "unigram_cost").unwrap();
    writeln!(model_file, "1.0 1.5 2.0 2.5 3.0").unwrap();
    writeln!(model_file, "bigram_cost").unwrap();
    for _ in 0..80 {
        writeln!(model_file, "1.0").unwrap();
    }
    writeln!(model_file, "row").unwrap();
    writeln!(model_file, "0 0 1 1 2").unwrap();
    writeln!(model_file, "hand").unwrap();
    writeln!(model_file, "0 0 0 1 1").unwrap();
    writeln!(model_file, "finger").unwrap();
    writeln!(model_file, "0 1 0 1 2").unwrap();
    model_file.flush().unwrap();

    let model = load_model(model_file.path()).unwrap();
    let full = vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E];

    let non_fixed = model.without_fixed(&full);

    // Should preserve order: a, c, e (positions 0, 2, 4)
    assert_eq!(non_fixed, vec![Kc::A, Kc::C, Kc::E]);
}

#[test]
fn test_model_with_fixed_interleaves_correctly() {
    let mut model_file = NamedTempFile::new().unwrap();
    writeln!(model_file, "layout").unwrap();
    writeln!(model_file, "X X X X X X").unwrap();
    writeln!(model_file, "keys").unwrap();
    writeln!(model_file, "a b c d e f").unwrap();
    writeln!(model_file, "fixed").unwrap();
    writeln!(model_file, "a None c None e None").unwrap();
    writeln!(model_file, "unigram_cost").unwrap();
    writeln!(model_file, "1.0 1.5 2.0 2.5 3.0 3.5").unwrap();
    writeln!(model_file, "bigram_cost").unwrap();
    for _ in 0..80 {
        writeln!(model_file, "1.0").unwrap();
    }
    writeln!(model_file, "row").unwrap();
    writeln!(model_file, "0 0 1 1 2 2").unwrap();
    writeln!(model_file, "hand").unwrap();
    writeln!(model_file, "0 0 0 1 1 1").unwrap();
    writeln!(model_file, "finger").unwrap();
    writeln!(model_file, "0 1 0 1 0 1").unwrap();
    model_file.flush().unwrap();

    let model = load_model(model_file.path()).unwrap();
    let non_fixed = vec![Kc::B, Kc::D, Kc::F];

    let full = model.with_fixed(&non_fixed);

    // Should interleave: a, b, c, d, e, f
    assert_eq!(full, vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F]);
}

#[test]
fn test_model_format_with_special_chars() {
    let mut model_file = NamedTempFile::new().unwrap();
    writeln!(model_file, "layout").unwrap();
    writeln!(model_file, "X X X X").unwrap();
    writeln!(model_file, "keys").unwrap();
    writeln!(model_file, ", . / ;").unwrap();
    writeln!(model_file, "fixed").unwrap();
    writeln!(model_file, "None None None None").unwrap();
    writeln!(model_file, "unigram_cost").unwrap();
    writeln!(model_file, "1.0 1.0 1.0 1.0").unwrap();
    writeln!(model_file, "bigram_cost").unwrap();
    for _ in 0..80 {
        writeln!(model_file, "1.0").unwrap();
    }
    writeln!(model_file, "row").unwrap();
    writeln!(model_file, "0 0 1 1").unwrap();
    writeln!(model_file, "hand").unwrap();
    writeln!(model_file, "0 0 1 1").unwrap();
    writeln!(model_file, "finger").unwrap();
    writeln!(model_file, "0 1 0 1").unwrap();
    model_file.flush().unwrap();

    let model = load_model(model_file.path()).unwrap();
    let layout = vec![Kc::Comma, Kc::Dot, Kc::Slash, Kc::Semicolon];

    let formatted = model.format(&layout);

    assert!(formatted.contains(','));
    assert!(formatted.contains('.'));
    assert!(formatted.contains('/'));
    assert!(formatted.contains(';'));
}

#[test]
fn test_model_consistency_checks() {
    let (model_file, _seeds, _uni, _bi, _tri) = create_complete_test_environment();
    let model = load_model(model_file.path()).unwrap();

    // All arrays should have consistent sizes
    assert_eq!(model.universe.len(), model.fixed.len());
    assert_eq!(model.universe.len(), model.unigram_cost.len());
    assert_eq!(model.universe.len(), model.row.len());
    assert_eq!(model.universe.len(), model.hand.len());
    assert_eq!(model.universe.len(), model.finger.len());

    // Bigram cost should have some values
    assert!(!model.bigram_cost.is_empty(), "Bigram cost array should not be empty");
}

#[test]
fn test_seed_layouts_valid_for_model() {
    let (model_file, seed_file, _uni, _bi, _tri) = create_complete_test_environment();

    let model = load_model(model_file.path()).unwrap();
    let seeds = load_seeds(seed_file.path()).unwrap();

    for seed in &seeds {
        // Each seed should have the same number of keys as the model universe
        assert_eq!(seed.0.len(), model.universe.len());

        // Each seed should only contain keys from the universe
        for key in &seed.0 {
            assert!(model.universe.contains(key), "Seed contains key not in universe: {:?}", key);
        }
    }
}

#[test]
fn test_model_geometric_consistency() {
    let (model_file, _seeds, _uni, _bi, _tri) = create_complete_test_environment();
    let model = load_model(model_file.path()).unwrap();

    // Verify row values are consistent
    for &row in &model.row {
        assert!(row <= 2, "Row should be 0, 1, or 2");
    }

    // Verify hand values are binary
    for &hand in &model.hand {
        assert!(hand == 0 || hand == 1, "Hand should be 0 or 1");
    }

    // Verify finger values are in valid range
    for &finger in &model.finger {
        assert!(finger <= 3, "Finger should be 0-3");
    }
}

#[test]
fn test_roundtrip_with_fixed_and_without_fixed() {
    let mut model_file = NamedTempFile::new().unwrap();
    writeln!(model_file, "layout").unwrap();
    writeln!(model_file, "X X X X X X X X").unwrap();
    writeln!(model_file, "keys").unwrap();
    writeln!(model_file, "a b c d e f g h").unwrap();
    writeln!(model_file, "fixed").unwrap();
    writeln!(model_file, "None b None d None f None h").unwrap();
    writeln!(model_file, "unigram_cost").unwrap();
    writeln!(model_file, "1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0").unwrap();
    writeln!(model_file, "bigram_cost").unwrap();
    for _ in 0..80 {
        writeln!(model_file, "1.0").unwrap();
    }
    writeln!(model_file, "row").unwrap();
    writeln!(model_file, "0 0 0 0 1 1 1 1").unwrap();
    writeln!(model_file, "hand").unwrap();
    writeln!(model_file, "0 0 0 0 1 1 1 1").unwrap();
    writeln!(model_file, "finger").unwrap();
    writeln!(model_file, "0 1 2 3 0 1 2 3").unwrap();
    model_file.flush().unwrap();

    let model = load_model(model_file.path()).unwrap();

    // Test roundtrip for multiple layouts
    let test_layouts = vec![
        vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F, Kc::G, Kc::H],
        vec![Kc::H, Kc::B, Kc::G, Kc::D, Kc::F, Kc::F, Kc::E, Kc::H],
        vec![Kc::C, Kc::B, Kc::A, Kc::D, Kc::G, Kc::F, Kc::E, Kc::H],
    ];

    for layout in test_layouts {
        let without = model.without_fixed(&layout);
        let reconstructed = model.with_fixed(&without);

        assert_eq!(reconstructed, layout, "Roundtrip should preserve layout");
    }
}

#[test]
fn test_histograms_structure() {
    let (_model_file, _seed_file, uni, bi, tri) = create_complete_test_environment();

    let hist = memelay::ingest::load_histograms(uni.path(), bi.path(), tri.path()).unwrap();

    // Verify histograms are vectors of tuples
    assert!(!hist.unigrams.is_empty());
    assert!(!hist.bigrams.is_empty());
    assert!(!hist.trigrams.is_empty());

    // Verify unigrams have correct structure
    for (kc, freq) in &hist.unigrams {
        assert!(*freq > 0.0);
        assert_ne!(*kc, Kc::None);
    }

    // Verify bigrams have correct structure
    for ((kc1, kc2), freq) in &hist.bigrams {
        assert!(*freq > 0.0);
        assert_ne!(*kc1, Kc::None);
        assert_ne!(*kc2, Kc::None);
    }

    // Verify trigrams have correct structure
    for ((kc1, kc2, kc3), freq) in &hist.trigrams {
        assert!(*freq > 0.0);
        assert_ne!(*kc1, Kc::None);
        assert_ne!(*kc2, Kc::None);
        assert_ne!(*kc3, Kc::None);
    }
}
