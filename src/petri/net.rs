use crate::{Place, Transition};

pub struct Tapn<'a> {
    pub places: Vec<Place>,
    pub transitions: Vec<Transition<'a>>,
}