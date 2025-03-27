use std::{cell::RefCell, rc::Rc};

use super::Place;

pub enum ArcType {
    Input(InputArc),
    Transport(TransportArc),
    Inhibitor(InhibitorArc),
}

impl ArcType {
    pub fn fire(&mut self) -> Vec<f64> {
        match self {
            ArcType::Input(arc) => arc.fire(),
            ArcType::Transport(arc) => arc.fire(),
            ArcType::Inhibitor(arc) => arc.fire(),
        }
    }

    pub fn can_fire(&self) -> bool {
        match self {
            ArcType::Input(arc) => arc.can_fire(),
            ArcType::Transport(arc) => arc.can_fire(),
            ArcType::Inhibitor(arc) => arc.can_fire(),
        }
    }
}

pub struct InputArc {
    pub input: Rc<RefCell<Place>>,
    pub weight: usize,
    pub timing: [f64; 2],
}

impl InputArc {
    pub fn fire(&mut self) -> Vec<f64> {
        if self.input.borrow_mut().invariants_hold(self.weight) {
            self.input.borrow_mut().remove_tokens(self.weight);
        }
        println!("Input arc fired");
        vec![0.0; self.weight]
    }

    pub fn can_fire(&self) -> bool {
        if self
            .input
            .borrow_mut()
            .tokens_hold(self.weight, &self.timing)
        {
            return self.input.borrow_mut().invariants_hold(self.weight);
        }
        false
    }
}

pub struct TransportArc {
    pub input: Rc<RefCell<Place>>,
    pub weight: usize,
    pub timing: [f64; 2],
}

impl TransportArc {
    pub fn fire(&mut self) -> Vec<f64> {
        let mut tokens: Vec<f64> = Vec::new();
        if self.input.borrow_mut().invariants_hold(self.weight) {
            tokens = self.input.borrow_mut().remove_tokens(self.weight);
        }
        println!("Transport arc fired");
        tokens
    }

    pub fn can_fire(&self) -> bool {
        if self
            .input
            .borrow_mut()
            .tokens_hold(self.weight, &self.timing)
        {
            return self.input.borrow_mut().invariants_hold(self.weight);
        }
        false
    }
}

pub struct InhibitorArc {
    pub input: Rc<RefCell<Place>>,
    pub weight: usize,
    pub constraint: usize,
    pub timing: [f64; 2],
}

impl InhibitorArc {
    pub fn fire(&mut self) -> Vec<f64> {
        if self.input.borrow_mut().invariants_hold(self.weight) {
            self.input.borrow_mut().remove_tokens(self.weight);
        }
        println!("Inhibitor arc fired");
        vec![0.0; self.weight]
    }

    pub fn can_fire(&self) -> bool {
        if self
            .input
            .borrow_mut()
            .tokens_hold(self.weight, &self.timing)
        {
            return self.input.borrow_mut().invariants_hold(self.weight);
        }
        false
    }
}

pub enum OutputArc {
    TransportArc(TransportOutputArc),
    Regular(RegularOutputArc),
}

pub struct TransportOutputArc {
    pub output: Rc<RefCell<Place>>,
    pub weight: usize,
}

impl TransportOutputArc {
    pub fn fire(&mut self, tokens: &[f64]) -> bool {
        if tokens.len() >= self.weight {
            self.output.borrow_mut().add_tokens(&tokens[..self.weight]);
            true
        } else {
            false
        }
    }
}

pub struct RegularOutputArc {
    pub output: Rc<RefCell<Place>>,
    pub weight: usize,
}

impl RegularOutputArc {
    pub fn fire(&mut self, tokens: &[f64]) -> bool {
        if tokens.len() >= self.weight {
            self.output.borrow_mut().add_tokens(&vec![0.0; self.weight]); // Example: Generate new tokens
            true
        } else {
            false
        }
    }
}
