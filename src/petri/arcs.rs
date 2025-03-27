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
        if self.can_fire() {
            let removed = self.input.borrow_mut().remove_tokens(self.weight);
            println!("Input arc fired, removed tokens: {:?}", removed);
            // Input arcs create new 0-age tokens
            vec![0.0; self.weight]
        } else {
            vec![]
        }
    }

    pub fn can_fire(&self) -> bool {
        self.input.borrow().tokens_hold(self.weight, &self.timing) &&
        self.input.borrow().invariants_hold(self.weight)
    }
}

pub struct TransportArc {
    pub input: Rc<RefCell<Place>>,
    pub weight: usize,
    pub timing: [f64; 2],
}

impl TransportArc {
    pub fn fire(&mut self) -> Vec<f64> {
        if self.can_fire() {
            let tokens = self.input.borrow_mut().remove_tokens(self.weight);
            println!("Transport arc fired, transporting tokens: {:?}", tokens);
            tokens
        } else {
            vec![]
        }
    }

    pub fn can_fire(&self) -> bool {
        self.input.borrow().tokens_hold(self.weight, &self.timing) &&
        self.input.borrow().invariants_hold(self.weight)
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
        // Inhibitor arcs don't actually consume tokens when firing
        println!("Inhibitor arc fired (no tokens consumed)");
        vec![]
    }

    pub fn can_fire(&self) -> bool {
        // Inhibitor checks if place has LESS tokens than constraint
        self.input.borrow().tokens.len() < self.constraint &&
        self.input.borrow().invariants_hold(0) // Check invariants for existing tokens
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
            // Transport arcs preserve the original token ages
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
        // Regular arcs create new 0-age tokens regardless of input
        // (but we use the weight from the arc definition)
        self.output.borrow_mut().add_tokens(&vec![0.0; self.weight]);
        true
    }
}
