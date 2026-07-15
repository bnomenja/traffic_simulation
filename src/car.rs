use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::consts::*;
use crate::light::TrafficController;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North, 
    South, 
    East,  
    West, 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Car {
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
impl Direction {
    //generate a random direction
    pub fn random() -> Self {
    }
}


impl Car {
    //generate a new car based on the direction of the input 
    pub fn new(direction: Direction) -> Self {
    }

    // change the position of the car
    pub fn update(&mut self, dt: f32, is_green: bool, ahead_progress: Option<f32>) {
    }

    // draw a single car
    pub fn draw(&self) {
    }

}

//manage all the cars and avoid spawn spam
pub struct CarManager {

    pub cars: Vec<Car>,
    last_spawn: [f64; 4], // last spawn for each direction

}

impl CarManager {

    pub fn new() -> Self {
    }

    // try to spawn a car if it is allowed
    pub fn try_spawn_car(&mut self, direction: Direction) {
    }
    
    // update the position of each cars, if it is out of the simuation, remove it.
    pub fn update(&mut self, t: f32, , lights: &TrafficController) {
    }

    // draw all the cars
    pub fn draw(&self) {
    }

}