use memelay::eval::{KeyState, LayoutEval};
use memelay::ingest::{load_histograms, load_model};
use memelay::types::Kc;
use memega::eval::Evaluator;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_minimal_test_model() -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "layout").unwrap();
    writeln!(file, "X X X X X X").unwrap();
    writeln!(file, "keys").unwrap();
    writeln!(file, "a b c d e f").unwrap();
    writeln!(file, "fixed").unwrap();
    writeln!(file, "None None None None None None").unwrap();
    writeln!(file, "unigram_cost").unwrap();
    writeln!(file, "1.0 1.5 2.0 2.5 3.0 3.5").unwrap();
    writeln!(file, "bigram_cost").unwrap();
    // Need exactly 80 bigram costs
    for _ in 0..80 {
        writeln!(file, "1.0").unwrap();
    }
    writeln!(file, "row").unwrap();
    writeln!(file, "0 0 1 1 2 2").unwrap();
    writeln!(file, "hand").unwrap();
    writeln!(file, "0 0 0 1 1 1").unwrap();
    writeln!(file, "finger").unwrap();
    writeln!(file, "0 1 0 1 0 1").unwrap();
    file.flush().unwrap();
    file
}

fn create_test_histograms() -> (NamedTempFile, NamedTempFile, NamedTempFile) {
    let mut uni = NamedTempFile::new().unwrap();
    writeln!(uni, "header").unwrap();
    writeln!(uni, "a 10.0").unwrap();
    writeln!(uni, "b 8.0").unwrap();
    writeln!(uni, "c 6.0").unwrap();
    writeln!(uni, "d 4.0").unwrap();
    writeln!(uni, "e 2.0").unwrap();
    writeln!(uni, "f 1.0").unwrap();
    uni.flush().unwrap();

    let mut bi = NamedTempFile::new().unwrap();
    writeln!(bi, "header").unwrap();
    writeln!(bi, "a b 5.0").unwrap();
    writeln!(bi, "b c 3.0").unwrap();
    writeln!(bi, "c d 2.0").unwrap();
    bi.flush().unwrap();

    let mut tri = NamedTempFile::new().unwrap();
    writeln!(tri, "header").unwrap();
    writeln!(tri, "a b c 1.0").unwrap();
    tri.flush().unwrap();

    (uni, bi, tri)
}

#[test]
fn test_evaluator_creation() {
    let model_file = create_minimal_test_model();
    let (uni, bi, tri) = create_test_histograms();

    let model = load_model(model_file.path()).unwrap();
    let hist = load_histograms(uni.path(), bi.path(), tri.path()).unwrap();

    let evaluator = LayoutEval {
        model,
        hist,
        match_keys: vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F],
    };

    // Verify evaluator was created successfully
    assert_eq!(evaluator.model.universe.len(), 6);
}

#[test]
fn test_evaluator_fitness_basic() {
    let model_file = create_minimal_test_model();
    let (uni, bi, tri) = create_test_histograms();

    let model = load_model(model_file.path()).unwrap();
    let hist = load_histograms(uni.path(), bi.path(), tri.path()).unwrap();

    let evaluator = LayoutEval {
        model,
        hist,
        match_keys: vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F],
    };

    let layout = KeyState(vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F]);
    let fitness = evaluator.fitness(&layout, &()).unwrap();

    // Fitness should be a positive number (lower is better)
    assert!(fitness > 0.0, "Fitness should be positive");
    assert!(fitness.is_finite(), "Fitness should be finite");
}

#[test]
fn test_evaluator_mutate() {
    let model_file = create_minimal_test_model();
    let (uni, bi, tri) = create_test_histograms();

    let model = load_model(model_file.path()).unwrap();
    let hist = load_histograms(uni.path(), bi.path(), tri.path()).unwrap();

    let evaluator = LayoutEval {
        model,
        hist,
        match_keys: vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F],
    };

    let original = KeyState(vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F]);
    let mut mutated = original.clone();

    // Test all mutation strategies (0-2)
    for strategy in 0..3 {
        evaluator.mutate(&mut mutated, 0.5, strategy);

        // After mutation, the layout should still have the same keys
        assert_eq!(mutated.len(), 6);

        // Verify all original keys are still present (just reordered)
        for key in &original.0 {
            assert!(mutated.0.contains(key), "Mutation should preserve all keys");
        }
    }
}

#[test]
fn test_evaluator_crossover() {
    let model_file = create_minimal_test_model();
    let (uni, bi, tri) = create_test_histograms();

    let model = load_model(model_file.path()).unwrap();
    let hist = load_histograms(uni.path(), bi.path(), tri.path()).unwrap();

    let evaluator = LayoutEval {
        model,
        hist,
        match_keys: vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F],
    };

    let parent1 = KeyState(vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F]);
    let parent2 = KeyState(vec![Kc::F, Kc::E, Kc::D, Kc::C, Kc::B, Kc::A]);

    // Test all crossover strategies (0-3)
    for strategy in 0..4 {
        let mut p1 = parent1.clone();
        let mut p2 = parent2.clone();

        evaluator.crossover(&mut p1, &mut p2, strategy);

        // After crossover, layouts should still have the same size
        assert_eq!(p1.len(), 6);
        assert_eq!(p2.len(), 6);

        // Verify all keys are still present in each layout
        for key in &parent1.0 {
            assert!(p1.0.contains(key), "Crossover should preserve all keys in p1");
            assert!(p2.0.contains(key), "Crossover should preserve all keys in p2");
        }
    }
}

#[test]
fn test_evaluator_distance() {
    let model_file = create_minimal_test_model();
    let (uni, bi, tri) = create_test_histograms();

    let model = load_model(model_file.path()).unwrap();
    let hist = load_histograms(uni.path(), bi.path(), tri.path()).unwrap();

    let evaluator = LayoutEval {
        model,
        hist,
        match_keys: vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F],
    };

    let layout1 = KeyState(vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F]);
    let layout2 = KeyState(vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F]);
    let layout3 = KeyState(vec![Kc::F, Kc::E, Kc::D, Kc::C, Kc::B, Kc::A]);

    let dist_same = evaluator.distance(&layout1, &layout2).unwrap();
    let dist_diff = evaluator.distance(&layout1, &layout3).unwrap();

    // Distance to identical layout should be 0
    assert_eq!(dist_same, 0.0);

    // Distance to completely reversed layout should be > 0
    assert!(dist_diff > 0.0);
}
