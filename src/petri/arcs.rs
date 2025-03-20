use super::{place, Place};

pub enum ArcType<'a> {
    Input(InputArc<'a>),
    Transport(TransportArc<'a>),
    Inhibitor(InhibitorArc<'a>),
}

impl<'a> ArcType<'a> {
    pub fn fire(&mut self) -> Vec<f64> {
        match self {
            ArcType::Input(arc) => arc.fire(),
            ArcType::Transport(arc) =>arc.fire(),
            ArcType::Inhibitor(arc) => arc.fire(),
        }
    }
}

pub struct InputArc<'a> {
    pub input: &'a mut Place,
    pub weight: usize,
}

impl<'a> InputArc<'a> {
    pub fn fire(&mut self) -> Vec<f64> {
        if self.input.invariants_hold(self.weight) {
            self.input.remove_tokens(self.weight);
        }
        println!("Input arc fired");
        vec![0.0; self.weight]
    }
}

pub struct TransportArc<'a> {
    pub input: &'a mut Place,
    pub weight: usize,
}

impl<'a> TransportArc<'a> {
    pub fn fire(&mut self) -> Vec<f64> {
        let mut tokens: Vec<f64> = Vec::new();
        if self.input.invariants_hold(self.weight) {
            tokens = self.input.remove_tokens(self.weight);
        }
        println!("Transport arc fired");
        tokens
    }
}

pub struct InhibitorArc<'a> {
    input: &'a mut Place,
    weight: usize,
    constraint: usize,
}

impl<'a> InhibitorArc<'a> {
    pub fn fire(&mut self) -> Vec<f64> {
        if self.input.invariants_hold(self.weight) {
            self.input.remove_tokens(self.weight);
        }
        println!("Inhibitor arc fired");
        vec![0.0; self.weight]
    }
}

pub enum OutputArc<'a> {
    TransportArc(TransportOutputArc<'a>),
    Regular(RegularOutputArc<'a>),
}

pub struct TransportOutputArc<'a> {
    pub output: &'a mut Place,
    pub weight: usize,
}

impl<'a> TransportOutputArc<'a> {
    pub fn fire(&mut self, tokens: &[f64]) -> bool {
        if tokens.len() >= self.weight {
            self.output.add_tokens(&tokens[..self.weight]);
            true
        } else {
            false
        }
    }
}

pub struct RegularOutputArc<'a> {
    pub output: &'a mut Place,
    pub weight: usize,
}

impl<'a> RegularOutputArc<'a> {
    pub fn fire(&mut self, tokens: &[f64]) -> bool {
        if tokens.len() >= self.weight {
            self.output.add_tokens(&vec![0.0; self.weight]); // Example: Generate new tokens
            true
        } else {
            false
        }
    }
}