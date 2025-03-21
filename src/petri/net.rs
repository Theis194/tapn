use crate::{Place, Transition};

pub struct Tapn<'a> {
    pub places: Vec<Place>,
    pub transitions: Vec<Transition<'a>>,
}

impl<'a> Tapn<'a> {
    pub fn step(&mut self) -> bool {
        // Step 1: Determine the next transition to fire
        let mut next_transition_index: Option<usize> = None;
        let mut min_firing_time = f64::INFINITY;

        for (index, transition) in self.transitions.iter_mut().enumerate() {
            if transition.is_ready() {
                if transition.firing_time < min_firing_time {
                    min_firing_time = transition.firing_time;
                    next_transition_index = Some(index);
                }
            }
        }

        if let Some(index) = next_transition_index {
            // Step 2: Delay the system
            let delay = self.transitions[index].firing_time;
            self.delay(delay);

            // Step 3: Update token ages
            self.update_token_ages(delay);

            // Step 4: Fire the transition
            self.transitions[index].fire();

            // Step 5: Update enabled transitions
            self.update_enabled_transitions();

            true
        } else {
            // No enabled transitions, simulation ends
            false
        }
    }

    fn delay(&mut self, delay: f64) {
        // Advance the simulation time by the delay
        // This can be tracked as a field in the Tapn struct if needed
    }

    fn update_token_ages(&mut self, delay: f64) {
        for place in &mut self.places {
            for token in &mut place.tokens {
                *token += delay;
            }
        }
    }

    fn update_enabled_transitions(&mut self) {
        for transition in &mut self.transitions {
            transition.is_ready();
        }
    }
}