pub trait SimulationObserver {
    fn on_step(&mut self, event: &SimulationEvent);
    fn on_completion(&mut self);
    fn should_stop(&self) -> bool;
}

pub enum SimulationEvent {
    TransitionFiring {
        transition_id: usize,
        firing_time: f64,
    },
    TransitionFired {
        transition_id: usize,
        firing_time: f64,
        tokens_consumed: Vec<f64>,
    },
    TokensChanged {
        place_id: usize,
        new_tokens: Vec<f64>,
    },
    TimeAdvanced {
        delta: f64,
        new_time: f64,
    },
}