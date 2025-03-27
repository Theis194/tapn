use std::collections::HashMap;

use super::{SimulationEvent, SimulationObserver};

pub struct TokenAgeObserver {
    pub age_distribution: HashMap<usize, Vec<f64>>,
    pub max_allowed_age: f64,
    pub should_stop: bool,
}

impl TokenAgeObserver {
    pub fn new(max_allowed_age: f64) -> Self {
        Self {
            age_distribution: HashMap::new(),
            max_allowed_age,
            should_stop: false,
        }
    }
}

impl SimulationObserver for TokenAgeObserver {
    fn on_step(&mut self, event: &super::SimulationEvent) {
        if let SimulationEvent::TokensChanged {
            place_id,
            new_tokens,
        } = event
        {
            for &age in new_tokens {
                if age > self.max_allowed_age {
                    self.should_stop = true;
                    break;
                }
            }

            self.age_distribution
                .entry(*place_id)
                .or_default()
                .extend(new_tokens);
        }
    }

    fn on_completion(&mut self) {
        
    }

    fn should_stop(&self) -> bool {
        self.should_stop
    }
}

pub struct TokenCoutObserver {
    pub place_thesholds: HashMap<usize, (usize, Comparison)>,
    pub should_stop: bool,
}

pub enum Comparison {
    LessThan,
    LessOrEqual,
    Equal,
    GreaterOrEqual,
    GreaterThan,
}

impl TokenCoutObserver {
    pub fn new() -> Self {
        Self {
            place_thesholds: HashMap::new(),
            should_stop: false,
        }
    }

    pub fn monitor_place(mut self, place_id: usize, threshold: usize, comparison: Comparison) -> Self {
        self.place_thesholds
            .insert(place_id, (threshold, comparison));
        self
    }

    fn check_condition(&self, count: usize, threshold: usize, comparison: &Comparison) -> bool {
        match comparison {
            Comparison::LessThan => count < threshold,
            Comparison::LessOrEqual => count <= threshold,
            Comparison::Equal => count == threshold,
            Comparison::GreaterOrEqual => count >= threshold,
            Comparison::GreaterThan => count > threshold,
        }
    }
}

impl SimulationObserver for TokenCoutObserver {
    fn on_step(&mut self, event: &SimulationEvent) {
        if let SimulationEvent::TokensChanged {
            place_id,
            new_tokens,
        } = event
        {
            if let Some((threshold, comparison)) = self.place_thesholds.get(place_id) {
                let token_count = new_tokens.len();
                //println!("token_count {}", token_count);
                if self.check_condition(token_count, *threshold, comparison) {
                    self.should_stop = true
                }
            }
        }
    }

    fn on_completion(&mut self) {
        
    }

    fn should_stop(&self) -> bool {
        self.should_stop
    }
}
