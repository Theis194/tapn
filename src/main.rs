use std::{cell::RefCell, rc::Rc};

use tapn::{
    observer::token_observers::TokenCoutObserver, petri::Distribution, ArcType, Comparison, InputArc, OutputArc, Place, RegularOutputArc, Tapn, Transition, TransportArc, TransportOutputArc
};

fn main() {
    let accumulated_time = Rc::new(RefCell::new(Place::new(
        0,
        "accumulated_time".to_string(),
        vec![0.0],
        vec![],
    )));
    let finished = Rc::new(RefCell::new(Place::new(
        0,
        "finished".to_string(),
        vec![],
        vec![],
    )));

    let delay_input = ArcType::Transport(TransportArc {
        input: Rc::clone(&accumulated_time),
        weight: 1,
        timing: [0.0, 999.0],
    });
    let delay_output = OutputArc::TransportArc(TransportOutputArc {
        output: Rc::clone(&accumulated_time),
        weight: 1,
    });

    let delay = Transition::new(vec![delay_input], vec![delay_output], Distribution::Uniform(0.0, 1.0));

    let timeout_input = ArcType::Input(InputArc {
        input: Rc::clone(&accumulated_time),
        weight: 1,
        timing: [1.0, 999.0],
    });
    let timeout_output = OutputArc::Regular(RegularOutputArc {
        output: Rc::clone(&finished),
        weight: 1,
    });

    let timeout = Transition::new(vec![timeout_input], vec![timeout_output], Distribution::Constant(0.0));

    let observer = Box::new(TokenCoutObserver::new().monitor_place(1, 1, Comparison::Equal));

    let mut euler = Tapn {
        places: vec![accumulated_time, finished],
        transitions: vec![delay, timeout],
        observers: vec![observer],
    };

    euler.run();
}
