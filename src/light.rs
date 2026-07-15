use macroquad::prelude::*;
use crate::consts::*;
use crate::car::{Car, Direction};

#[derive(Debug, Clone)]
pub struct TrafficController {
    active: Direction,
    next_active: Direction,
    timer: f64,
    clearing: bool,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::South,
    Direction::West,
    Direction::North,
    Direction::East,
];

fn lane_capacity() -> usize {
    let lane_length = 400.0_f32;
    (lane_length / (CAR_WIDTH + SAFETY_GAP)).floor() as usize
}

impl TrafficController {
    pub fn new() -> Self {
        Self {
            active: Direction::South,
            next_active: Direction::West,
            timer: 1.0,
            clearing: false,
        }
    }

    pub fn is_green(&self, direction: Direction) -> bool {
        !self.clearing && direction == self.active
    }

    pub fn update(&mut self, dt: f64, cars: &[Car]) {
        if self.timer > 0.0 {
            self.timer -= dt;
        }

        let south = cars.iter().filter(|c| c.direction == Direction::South).count();
        let west = cars.iter().filter(|c| c.direction == Direction::West).count();
        let north = cars.iter().filter(|c| c.direction == Direction::North).count();
        let east = cars.iter().filter(|c| c.direction == Direction::East).count();
        let counts = [south, west, north, east];
        let capacity = lane_capacity();

        if self.clearing {
            if self.timer <= 0.0 {
                self.active = self.next_active;
                self.clearing = false;

                let count = match self.active {
                    Direction::South => south,
                    Direction::West => west,
                    Direction::North => north,
                    Direction::East => east,
                };

                let ratio = count as f64 / capacity as f64;
                self.timer = if ratio > 0.4 {
                    2.0
                } else if count > 0 {
                    1.0
                } else {
                    0.5
                };
            }
        } else {
            if self.timer <= 0.0 {
                self.next_active = self.pick_next(&counts);
                self.clearing = true;
                self.timer = 0.5;
            }
        }
    }

    fn pick_next(&self, counts: &[usize; 4]) -> Direction {
        let current_idx = DIRECTIONS.iter().position(|&d| d == self.active).unwrap_or(0);

        // Find the most congested lane, excluding the currently active one
        let mut best_idx = (current_idx + 1) % 4;
        let mut best_count = 0;
        for (i, &cnt) in counts.iter().enumerate() {
            if i != current_idx && cnt > best_count {
                best_count = cnt;
                best_idx = i;
            }
        }

        DIRECTIONS[best_idx]
    }

    pub fn draw(&self) {
        let mv = WINDOW_WIDTH as f32 / 2.0;
        let mh = WINDOW_HEIGHT as f32 / 2.0;
        let gap = LANE_WIDTH;
        let r = TRAFFIC_LIGHT_SIZE / 2.0;

        let tl = if !self.clearing && self.active == Direction::South { GREEN } else { RED };
        let tr = if !self.clearing && self.active == Direction::West { GREEN } else { RED };
        let bl = if !self.clearing && self.active == Direction::East { GREEN } else { RED };
        let br = if !self.clearing && self.active == Direction::North { GREEN } else { RED };

        draw_circle(mv - gap - 15.0, mh - gap - 15.0, r, tl);
        draw_circle(mv + gap + 15.0, mh - gap - 15.0, r, tr);
        draw_circle(mv - gap - 15.0, mh + gap + 15.0, r, bl);
        draw_circle(mv + gap + 15.0, mh + gap + 15.0, r, br);
    }
}
