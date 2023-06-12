use crate::lib::Simulation;

mod lib;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const NUM_POINTS: usize = 10;

fn main() {
    let mut sim: Simulation<WIDTH, HEIGHT> = Simulation::new(NUM_POINTS);
    sim.start();
}