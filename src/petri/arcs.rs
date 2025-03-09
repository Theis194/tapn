use super::Place;

pub enum ArcType {
    Input(InputArc),
    Transport(TransportArc),
    Inhibitor(InhibitorArc),
}

impl ArcType {
    pub fn fire(&mut self) -> bool {
        match self {
            ArcType::Input(arc) => arc.fire(),
            ArcType::Transport(arc) =>arc.fire(),
            ArcType::Inhibitor(arc) => arc.fire(),
        }
    }
}

pub struct InputArc {
    input: Place,
    weight: usize,
}

impl InputArc {
    pub fn fire(&mut self) -> bool {
        if self.input.invariants_hold(self.weight) {
            self.input.remove_tokens(self.weight);
        }
        println!("Input arc fired");
        true
    }
}

pub struct TransportArc {
    input: Place,
    weight: usize,
}

impl TransportArc {
    pub fn fire(&self) -> bool {
        println!("Transport arc fired");
        true
    }
}

pub struct InhibitorArc {
    input: Place,
    weight: usize,
    constraint: usize,
}

impl InhibitorArc {
    pub fn fire(&self) -> bool {
        println!("Inhibitor arc fired");
        true
    }
}

pub struct OutputArc {
    output: Place,
    weight: usize,
}

impl OutputArc {
    pub fn fire(&self) -> bool {
        println!("Output arc fired");
        true
    }
}