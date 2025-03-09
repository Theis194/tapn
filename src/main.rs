use tapn::{ Invariant, Place };

fn main() {
    let place = Place::new(
        String::from("test"),
        vec![0.0],
        vec![Invariant::new(|age: f64| { age < 3.0 })]
    );

    println!("{:?}", place.invariants_hold());
}
