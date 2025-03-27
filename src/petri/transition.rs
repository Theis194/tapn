use super::{OutputArc, arcs::ArcType};
use rand::Rng;
use rand::distr::Uniform;

pub struct Transition {
    pub input_arcs: Vec<ArcType>,
    pub output_arcs: Vec<OutputArc>,
    pub distribution_function: Distribution,
    pub firing_time: f64,
}

impl Transition {
    pub fn new(input_arcs: Vec<ArcType>, output_arcs: Vec<OutputArc>, distribution_function: Distribution) -> Transition {
        Transition {
            input_arcs,
            output_arcs,
            distribution_function,
            firing_time: 0.0,
        }
    }

    pub fn fire(&mut self) -> bool {
        let mut transport_tokens: Vec<Vec<f64>> = Vec::new();
        let mut other_tokens: Vec<f64> = Vec::new();

        for input_arc in &mut self.input_arcs {
            match input_arc {
                ArcType::Transport(arc) => {
                    let tokens = arc.fire();
                    transport_tokens.push(tokens);
                }
                _ => {
                    let tokens = input_arc.fire();
                    other_tokens.extend(tokens);
                }
            }
        }

        let mut transport_index = 0;
        for output_arc in &mut self.output_arcs {
            match output_arc {
                OutputArc::TransportArc(arc) => {
                    if transport_index < transport_tokens.len() {
                        arc.fire(&transport_tokens[transport_index]);
                        transport_index += 1;
                    }
                }
                OutputArc::Regular(arc) => {
                    arc.fire(&other_tokens);
                }
            }
        }

        true
    }

    pub fn is_ready(&mut self) -> bool {
        let mut is_ready = true;
        for input_arc in &self.input_arcs {
            is_ready = input_arc.can_fire();
            if !is_ready {
                return false;
            }
        }

        self.generate_firing_time();

        is_ready
    }

    fn generate_firing_time(&mut self) {
        self.firing_time = self.distribution_function.sample();
    }
}

pub enum Distribution {
    Constant(f64),
    Uniform(f64, f64),
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
        }
    }
}
