use crate::Field;

pub struct Simulation {
    pub field: Field,
    pub speed: u64,
    pub wanted_happiness: f32,
    pub running: bool,
}
