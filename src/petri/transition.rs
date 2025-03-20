use super::{arcs::ArcType, OutputArc};

pub struct Transition<'a> {
    pub input_arcs: Vec<ArcType<'a>>,
    pub output_arcs: Vec<OutputArc<'a>>,
}

impl<'a> Transition<'a> {
    pub fn fire(&mut self) -> bool {
        let mut transport_tokens: Vec<Vec<f64>> = Vec::new();
        let mut other_tokens: Vec<f64> = Vec::new();

        for input_arc in &mut self.input_arcs {
            match input_arc {
                ArcType::Transport(arc) => {
                    let tokens = arc.fire();
                    transport_tokens.push(tokens);
                },
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
                },
                OutputArc::Regular(arc) => {
                    arc.fire(&other_tokens);
                }
            }
        }

        true
    }
}