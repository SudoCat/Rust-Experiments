enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Import Specific
use TrafficLight::{Red, Yellow};
// Import all
use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}