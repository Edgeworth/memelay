use average::{Mean, Variance};
use derive_more::Display;
use eyre::{eyre, Result};
use mathru::statistics::test::{Test, T};

// Sample contains sampled values, e.g. times, distances, costs, etc.
#[derive(Debug, Display, Default, Clone, PartialOrd, PartialEq)]
#[display(fmt = "Sample(mean={:.2}, dev={:.2})", "self.mean()", "self.stddev()")]
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

    pub fn vec(&self) -> &Vec<f64> {
        &self.v
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn mean(&self) -> f64 {
        let v: Mean = self.v.iter().collect();
        v.mean()
    }

    pub fn stddev(&self) -> f64 {
        self.variance().sqrt()
    }

    pub fn variance(&self) -> f64 {
        let v: Variance = self.v.iter().collect();
        v.sample_variance()
    }

    pub fn ttest(&self, o: &Sample) -> Result<f64> {
        if self.variance() == 0.0 || o.variance() == 0.0 {
            Err(eyre!("variance is zero"))
        } else {
            let v = T::test_independence_unequal_variance(&self.v, &o.v);
            Ok(v.p_value())
        }
    }
}
