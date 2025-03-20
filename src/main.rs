use tapn::{
    ArcType, InputArc, Invariant, OutputArc, Place, RegularOutputArc, Transition, TransportArc,
    TransportOutputArc,
};

fn main() {
    let place = Place::new(
        String::from("test"),
        vec![3.0, 0.0],
        vec![Invariant::new(|age: f64| age < 3.0)],
    );

    println!("{:?}", place.invariants_hold(1));

    let mut input_place1 = Place::new(
        String::from("test"),
        vec![1.0, 2.0, 3.0],
        vec![Invariant::new(|_| true)],
    );
    let mut input_place2 = Place::new(
        String::from("test"),
        vec![4.0, 5.0],
        vec![Invariant::new(|_| true)],
    );
    let mut output_place1 = Place::new(
        String::from("test"),
        Vec::new(),
        vec![Invariant::new(|_| true)],
    );
    let mut output_place2 = Place::new(
        String::from("test"),
        Vec::new(),
        vec![Invariant::new(|_| true)],
    );

    let input_arc = ArcType::Input(InputArc {
        input: &mut input_place1,
        weight: 2,
    });

    let transport_arc = ArcType::Transport(TransportArc {
        input: &mut input_place2,
        weight: 1,
    });

    let transport_output_arc = OutputArc::TransportArc(TransportOutputArc {
        output: &mut output_place1,
        weight: 1,
    });

    let regular_output_arc = OutputArc::Regular(RegularOutputArc {
        output: &mut output_place2,
        weight: 2,
    });

    let mut transition = Transition {
        input_arcs: vec![input_arc, transport_arc],
        output_arcs: vec![transport_output_arc, regular_output_arc],
    };

    transition.fire();

    println!("Output place 1 tokens: {:?}", output_place1.tokens); // Should contain tokens from transport arc
    println!("Output place 2 tokens: {:?}", output_place2.tokens); // Should contain new tokens
}
