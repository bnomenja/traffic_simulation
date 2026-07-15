use macroquad::prelude::*;
use crate::consts::*;
use crate::car::{Car, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightState {
    Red,
    Green,
}

// handling which light should turn green to avoid colisions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrafficController {
    active: Direction,   // which direction is green;
    last_change: f64,    // last time we changed the green spot; 
}

impl TrafficController {
    //pub fn new() -> Self {
    //}

    // light logic based on the time and the actual cars situations
    pub fn update(&mut self, dt: f64, cars: &[Car]) {
    }

    //draw the lights in the simulation
    pub fn draw(&self) {
    }
}