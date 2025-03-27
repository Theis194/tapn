use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Place, SimulationEvent, SimulationObserver, Transition};

pub struct Tapn {
    pub places: Vec<Rc<RefCell<Place>>>,
    pub transitions: Vec<Transition>,
    pub observers: Vec<Box<dyn SimulationObserver>>,
    pub steps: usize,
    pub current_time: f64,
    pub transition_firings: HashMap<usize, usize>,
}

impl Tapn {
    pub fn new(
        places: Vec<Rc<RefCell<Place>>>,
        transitions: Vec<Transition>,
    ) -> Self {
        Self {
            places,
            transitions,
            observers: Vec::new(),
            steps: 0,
            current_time: 0.0,
            transition_firings: HashMap::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Box<dyn SimulationObserver>) {
        self.observers.push(observer);
    }

    pub fn notify_observers(&mut self, event: SimulationEvent) {
        for observer in &mut self.observers {
            observer.on_step(&event);
        }
    }

    pub fn step(&mut self) -> bool {
        // Update enabled status of all transitions
        self.update_enabled_transitions();

        // Find all enabled transitions with their firing times
        let enabled_transitions: Vec<_> = self
            .transitions
            .iter_mut()
            .enumerate()
            .filter_map(|(i, t)| t.is_ready().then_some((i, t.firing_time)))
            .collect();

        if enabled_transitions.is_empty() {
            return false;
        }

        // Handle urgent transitions first
        if let Some(&(urgent_idx, _)) = enabled_transitions
            .iter()
            .find(|&&(i, _)| self.transitions[i].urgent)
        {
            return self.fire_transition(urgent_idx);
        }

        // Find the transition with minimal firing time
        let (transition_idx, delay) = enabled_transitions
            .iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        self.delay(*delay);
        self.fire_transition(*transition_idx)
    }

    fn fire_transition(&mut self, index: usize) -> bool {
        let firing_time = self.transitions[index].firing_time;

        // Notify before firing
        let pre_event = SimulationEvent::TransitionFiring {
            transition_id: index,
            firing_time,
        };
        self.notify_observers(pre_event);

        // Update token ages
        self.update_token_ages(firing_time);

        // Fire the transition and capture consumed tokens
        let consumed_tokens = {
            // Isolate the transition borrow
            let transition = &mut self.transitions[index];
            transition.fire()
        };

        // Record the firing
        *self.transition_firings.entry(index).or_insert(0) += 1;
        self.steps += 1;

        // Collect place states after firing
        let place_states: HashMap<_, _> = self
            .places
            .iter()
            .map(|p| {
                let place = p.borrow();
                (place.id, place.tokens.clone())
            })
            .collect();

        // Notify about token changes
        for (place_id, tokens) in place_states {
            self.notify_observers(SimulationEvent::TokensChanged {
                place_id,
                new_tokens: tokens,
            });
        }

        // Notify after firing
        let post_event = SimulationEvent::TransitionFired {
            transition_id: index,
            firing_time,
            tokens_consumed: consumed_tokens,
        };
        self.notify_observers(post_event);

        true
    }

    // Query method
    pub fn firing_count(&self, transition_id: usize) -> usize {
        *self.transition_firings.get(&transition_id).unwrap_or(&0)
    }

    pub fn run(&mut self) {
        while !self.should_continue() {
            self.step();
        }

        for observer in &mut self.observers {
            observer.on_completion();
        }
    }

    pub fn should_continue(&mut self) -> bool {
        self.observers.iter().any(|o| o.should_stop())
    }

    fn delay(&mut self, delay: f64) {
        // Advance the simulation time by the delay
        // This can be tracked as a field in the Tapn struct if needed
    }

    fn update_token_ages(&mut self, delay: f64) {
        if delay <= 0.0 {
            return;
        }

        self.current_time += delay;

        for place in &mut self.places {
            let mut place = place.borrow_mut();
            for token in &mut place.tokens {
                *token += delay;
            }
        }

        self.notify_observers(SimulationEvent::TimeAdvanced {
            delta: delay,
            new_time: self.current_time,
        });
    }

    fn update_enabled_transitions(&mut self) {
        for transition in &mut self.transitions {
            transition.is_ready();
        }
    }
}
