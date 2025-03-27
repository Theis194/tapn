use crate::Invariant;

#[derive(Clone)]
pub struct Place {
    pub id: usize,
    pub name: String,
    pub tokens: Vec<f64>,
    pub invariants: Vec<Invariant>,
}

impl Place {
    pub fn new(id: usize, name: String, tokens: Vec<f64>, invariants: Vec<Invariant>) -> Place {
        Place {
            id,
            name,
            tokens,
            invariants,
        }
    }

    pub fn invariants_hold(&self, n: usize) -> bool {
        // If `n` is greater than the number of tokens, it's impossible to satisfy the condition.
        if n > self.tokens.len() {
            return false;
        }

        // If there are no invariants then all tokens hold the invariant
        if self.invariants.len() == 0 {
            return true;
        }

        // Count how many tokens satisfy all invariants.
        let valid_tokens_count = self
            .tokens
            .iter()
            .filter(|token| {
                self.invariants
                    .iter()
                    .all(|invariant| invariant.check(**token)) // We have to de-refference the token twice because of filter
            })
            .count();

        // Check if at least `n` tokens satisfy all invariants.
        valid_tokens_count >= n
    }

    pub fn tokens_hold(&self, n: usize, timing: &[f64; 2]) -> bool {
        let min_age = timing[0];
        let max_age = timing[1];

        // Count tokens within age bounds
        let count = self
            .tokens
            .iter()
            .filter(|&age| *age >= min_age && *age <= max_age)
            .count();

        count >= n
    }

    pub fn remove_tokens(&mut self, n: usize) -> Vec<f64> {
        let mut tokens = Vec::with_capacity(n);

        // Early return if we don't have enough tokens
        if self.tokens.len() < n {
            return tokens;
        }

        // Sort tokens by age (youngest first)
        self.tokens
            .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Remove the n oldest tokens (from the end after sorting)
        tokens.extend(self.tokens.drain(self.tokens.len() - n..));

        tokens
    }

    pub fn add_tokens(&mut self, tokens: &[f64]) {
        self.tokens.extend_from_slice(tokens);
    }
}
