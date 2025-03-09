use crate::Invariant;

pub struct Place {
    name: String,
    tokens: Vec<f64>,
    invariants: Vec<Invariant>,
}

impl Place {
    pub fn new(name: String, tokens: Vec<f64>, invariants: Vec<Invariant>) -> Place {
        Place {
            name,
            tokens,
            invariants
        }
    }

    pub fn invariants_hold(&self, n: f64) -> bool {
        self.invariants
            .iter()
            .all(|invariant| {
                self.tokens
                    .iter()
                    .map(|token| invariant.check(*token))
                    .collect().len() > n
       })
    }
}