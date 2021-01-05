use crate::layout::Layout;
use radiate::Problem;

pub struct Fitness {}

impl Fitness {
    pub fn new() -> Self {
        Self {}
    }
}

impl Problem<Layout> for Fitness {
    fn empty() -> Self {
        Self::new()
    }

    fn solve(&self, model: &mut Layout) -> f32 {
        0.0
    }
}
