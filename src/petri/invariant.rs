#[derive(Clone, Copy)]
pub struct Invariant {
    condition: fn(token_age: f64) -> bool,
}

impl Invariant {
    pub fn new(condition: fn(token_age: f64) -> bool) -> Invariant {
        Invariant {
            condition
        }
    }

    pub fn check(&self, token_age: f64) -> bool {
        (self.condition)(token_age)
    }
}