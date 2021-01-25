#![warn(rust_2018_idioms, clippy::all)]
#![feature(
    const_fn,
    option_result_contains,
    trait_alias,
    iterator_fold_self,
    type_alias_impl_trait,
    partition_point,
    bool_to_option,
    map_first_last,
    option_unwrap_none,
    array_windows,
    array_chunks
)]

use std::collections::HashMap;

// Sample contains sampled values, e.g. times, distances, costs, etc.
#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct Sample {
    v: Vec<f64>,
}

impl Sample {
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }

    pub fn add(&mut self, v: f64) {
        self.v.push(v)
    }
}

// Group of samples of the same type to compare together.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SampleGroup {
    samples: HashMap<String, Sample>,
}

impl SampleGroup {
    pub fn new() -> Self {
        Self { samples: HashMap::new() }
    }

    // Adds the sampled value to the Sample with name |id|.
    pub fn add(&mut self, id: &str, v: f64) {
        self.samples.entry(id.to_owned()).or_insert_with(Sample::new).add(v);
    }
}

// Time-series like object.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Series;

// Grapher performs analysis and draws graphs of samples and sample groups.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Grapher {
    groups: HashMap<String, SampleGroup>,
}

// TODO: Some way to get number of samples required?
impl Grapher {
    pub fn new() -> Self {
        Self { groups: HashMap::new() }
    }

    // Adds the sampled value to the Sample with name |id| in group |group|.
    pub fn add(&mut self, group: &str, id: &str, v: f64) {
        println!("Add {} {} {}", group, id, v);
        self.groups.entry(group.to_owned()).or_insert_with(SampleGroup::new).add(id, v);
    }
}
