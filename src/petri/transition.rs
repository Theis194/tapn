use super::{OutputArc, arcs::ArcType};
use rand::distr::Uniform;
use rand::{Rng, rngs};
use rand_distr::{Exp, Normal};
pub struct Transition {
    pub input_arcs: Vec<ArcType>,
    pub output_arcs: Vec<OutputArc>,
    pub distribution_function: Distribution,
    pub firing_time: f64,
    pub urgent: bool,
    pub id: usize,
}

impl Transition {
    pub fn new(
        input_arcs: Vec<ArcType>,
        output_arcs: Vec<OutputArc>,
        distribution_function: Distribution,
        urgent: bool,
        id: usize,
    ) -> Transition {
        Transition {
            input_arcs,
            output_arcs,
            distribution_function,
            firing_time: 0.0,
            urgent,
            id,
        }
    }

    pub fn fire(&mut self) -> Vec<f64> {
        let mut consumed_tokens = Vec::new();

        // Process input arcs
        for arc in &mut self.input_arcs {
            match arc {
                ArcType::Input(arc) => {
                    let tokens = arc.fire();
                    consumed_tokens.extend(tokens);
                }
                ArcType::Transport(arc) => {
                    let tokens = arc.fire();
                    consumed_tokens.extend(tokens);
                }
                ArcType::Inhibitor(_) => {} // Inhibitor arcs don't consume tokens
            }
        }

        if self.output_arcs.is_empty() {
            return consumed_tokens;
        }

        // Process output arcs
        for arc in &mut self.output_arcs {
            match arc {
                OutputArc::TransportArc(arc) => {
                    arc.fire(&consumed_tokens);
                }
                OutputArc::Regular(arc) => {
                    // For regular arcs, we use the consumed tokens' ages
                    arc.fire(&consumed_tokens);
                }
            }
        }

        consumed_tokens
    }

    pub fn is_ready(&mut self) -> bool {
        // Check all input arcs
        for input_arc in &self.input_arcs {
            if !input_arc.can_fire() {
                return false;
            }
        }

        // Only generate firing time if not urgent
        if !self.urgent {
            self.generate_firing_time();
        } else {
            self.firing_time = 0.0;
        }

        true
    }

    fn generate_firing_time(&mut self) {
        self.firing_time = self.distribution_function.sample();
    }
}

pub enum Distribution {
    Constant(f64),
    Uniform(f64, f64),
    Normal(f64, f64),
    Exponential(f64),
}

impl Distribution {
    pub fn sample(&self) -> f64 {
        match self {
            Distribution::Constant(val) => *val,
            Distribution::Uniform(min, max) => {
                let mut rng = rand::rng();
                let range = Uniform::new(*min, *max).unwrap();
                rng.sample(range)
            }
            Distribution::Normal(mean, std_dev) => {
                let mut rng = rand::rng();
                let normal_range = Normal::new(*mean, *std_dev).unwrap();
                rng.sample(normal_range)
            }
            Distribution::Exponential(rate) => {
                let mut rng = rand::rng();
                let exp_range = Exp::new(rate).unwrap();
                rng.sample(exp_range)
            }
        }
    }
}
