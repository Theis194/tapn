use super::{arcs::ArcType, OutputArc};

pub struct Transition {
    input_arcs: Vec<ArcType>,
    output_arcs: Vec<OutputArc>,
}

impl Transition {
    pub fn fire(&mut self) -> bool {
        for input_arc in &mut self.input_arcs {
            if !input_arc.fire() {
                return false;
            }
        }

        for output_arc in &mut self.output_arcs {
            if !output_arc.fire() {
                return false;
            }
        }

        true
    }
}