use super::{OutputArc, arcs::ArcType};
use rand::Rng;
use rand::distr::Uniform;

pub struct Transition<'a> {
    pub input_arcs: Vec<ArcType<'a>>,
    pub output_arcs: Vec<OutputArc<'a>>,
    pub distribution_function: fn() -> f64,
    pub firing_time: f64,
}

impl<'a> Transition<'a> {
    pub fn new(input_arcs: Vec<ArcType<'a>>, output_arcs: Vec<OutputArc<'a>>) -> Transition<'a> {
        Transition {
            input_arcs,
            output_arcs,
            distribution_function: (|| 0.0),
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
        self.firing_time = (self.distribution_function)();
    }
}

pub enum Distribution {
    Constant,
    Uniform,
}

impl Distribution {
    pub fn sample(&self) -> f64 {
        match self {
            Distribution::Constant => 0.0,
            Distribution::Uniform => {
                let mut rng = rand::rng();
                let range = Uniform::new(0.0, 1.0).unwrap();
                let random_number = rng.sample(range);

                random_number
            }
        }
    }
}
