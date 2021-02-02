use average::Variance;
use mathru::{
    statistics::{
        test::{Test, T},
    },
};
use eyre::Result;
use eyre::eyre;

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

    pub fn vec(&self) -> &Vec<f64> {
        &self.v
    }

    pub fn variance(&self) -> f64 {
        let v: Variance = self.v.iter().collect();
        v.sample_variance()
    }

    pub fn ttest(&self, o: &Sample) -> Result<()> {
        if self.variance() == 0.0 || o.variance() == 0.0 {
            Err(eyre!("variance is zero"))
        } else {
        let t = T::test_independence_unequal_variance(&self.v, &o.v);
        println!("{}", t.value());
        Ok(())
        }
    }
}
