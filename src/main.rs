use std::{cell::RefCell, rc::Rc};

use tapn::{
    ArcType, Comparison, Distribution, InputArc, OutputArc, Place, RegularOutputArc, Tapn, TokenAgeObserver, TokenCoutObserver, Transition, TransportArc, TransportOutputArc
};

fn main() {
    // Create places with unique IDs
    let accumulated_time = Rc::new(RefCell::new(Place::new(
        0,  // ID 0
        "accumulated_time".to_string(),
        vec![0.0],  // Start with one token at age 0.0
        vec![],
    )));
    
    let finished = Rc::new(RefCell::new(Place::new(
        1,  // ID 1
        "finished".to_string(),
        vec![],  // Start empty
        vec![],
    )));

    // Create delay transition (non-urgent)
    let delay = Transition::new(
        vec![ArcType::Transport(TransportArc {
            input: Rc::clone(&accumulated_time),
            weight: 1,
            timing: [0.0, f64::INFINITY],
        })],
        vec![OutputArc::TransportArc(TransportOutputArc {
            output: Rc::clone(&accumulated_time),
            weight: 1,
        })],
        Distribution::Uniform(0.0, 1.0),
        false,
        0,  // Transition ID 0
    );

    // Create timeout transition (should be urgent)
    let timeout = Transition::new(
        vec![ArcType::Input(InputArc {
            input: Rc::clone(&accumulated_time),
            weight: 1,
            timing: [1.0, f64::INFINITY],  // Only fire when token age ≥1.0
        })],
        vec![OutputArc::Regular(RegularOutputArc {
            output: Rc::clone(&finished),
            weight: 1,
        })],
        Distribution::Constant(0.0),
        true,  // This should be urgent!
        1,  // Transition ID 1
    );

    // Create and configure observers
    let count_observer = Box::new(TokenCoutObserver::new()
        .monitor_place(1, 1, Comparison::Equal));  // Stop when place 1 (finished) has exactly 1 token
        
    let age_observer = Box::new(TokenAgeObserver::new(10.0));  // Emergency stop if any token age > 10.0

    // Create TAPN model
    let mut euler = Tapn::new(
        vec![accumulated_time, finished],
        vec![delay, timeout],
    );

    // Add observers
    euler.add_observer(count_observer);
    euler.add_observer(age_observer);

    // Run the simulation until an observer stops it
    euler.run();

    // Print final state
    println!("Simulation completed in {} steps", euler.steps);
    println!("Accumulated time tokens: {:?}", euler.places[0].borrow().tokens);
    println!("Finished tokens: {:?}", euler.places[1].borrow().tokens);
    
    // Print transition firing counts
    println!("Delay transition fired {} times", euler.firing_count(0));
    println!("Timeout transition fired {} times", euler.firing_count(1));
}