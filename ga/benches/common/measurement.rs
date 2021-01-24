use criterion::measurement::{Measurement, ValueFormatter};
use criterion::Throughput;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub struct F64Measurement {
    value: Rc<RefCell<f64>>,
}

impl F64Measurement {
    pub fn new(value: Rc<RefCell<f64>>) -> Self {
        Self { value }
    }
}

impl Measurement for F64Measurement {
    type Intermediate = f64;
    type Value = f64;

    fn start(&self) -> f64 {
        *self.value.borrow()
    }

    fn end(&self, st: f64) -> f64 {
        // Work around bug in criterion.rs where it expects there to be more than one value reported.
        *self.value.borrow() - st + rand::thread_rng().gen_range(1.0e-7..2.0e-7)
    }

    fn add(&self, v1: &f64, v2: &f64) -> f64 {
        *v1 + *v2
    }

    fn zero(&self) -> f64 {
        0.0
    }

    fn to_f64(&self, v: &f64) -> f64 {
        *v
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &F64Formatter
    }
}

struct F64Formatter;

impl ValueFormatter for F64Formatter {
    fn scale_values(&self, _: f64, _: &mut [f64]) -> &'static str {
        ""
    }

    fn scale_throughputs(&self, _: f64, _: &Throughput, _: &mut [f64]) -> &'static str {
        ""
    }

    fn scale_for_machines(&self, _: &mut [f64]) -> &'static str {
        ""
    }
}
