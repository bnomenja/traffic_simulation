use crate::car::{Car, Direction};
use crate::consts::*;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct TrafficController {
    active: Direction,
    next_active: Direction,
    timer: f64,
    clearing: bool,
    waiting_time: [f64; 4],
}

const DIRECTIONS: [Direction; 4] = [
    Direction::South,
    Direction::West,
    Direction::North,
    Direction::East,
];

fn lane_length(direction: Direction) -> f32 {
    let mv = WINDOW_WIDTH as f32 / 2.0;
    let mh = WINDOW_HEIGHT as f32 / 2.0;

    match direction {
        Direction::South => mh - 10.0,
        Direction::North => (WINDOW_HEIGHT as f32 - 35.0) - mh,
        Direction::East => mv - 10.0,
        Direction::West => (WINDOW_WIDTH as f32 - 35.0) - mv,
    }
}

fn lane_capacity(direction: Direction) -> usize {
    (lane_length(direction) / (CAR_WIDTH + SAFETY_GAP)).floor() as usize
}

impl TrafficController {
    pub fn new() -> Self {
        Self {
            active: Direction::South,
            next_active: Direction::West,
            timer: 0.5,
            clearing: false,
            waiting_time: [0.0; 4],
        }
    }

    pub fn is_green(&self, direction: Direction) -> bool {
        !self.clearing && direction == self.active
    }

    pub fn update(&mut self, dt: f64, cars: &[Car]) {
        if self.timer > 0.0 {
            self.timer -= dt;
        }

        for (i, &dir) in DIRECTIONS.iter().enumerate() {
            if !self.clearing && dir == self.active {
                self.waiting_time[i] = 0.0;
            } else {
                self.waiting_time[i] += dt;
            }
        }

        let south = cars
            .iter()
            .filter(|c| c.direction == Direction::South)
            .count();
        let west = cars
            .iter()
            .filter(|c| c.direction == Direction::West)
            .count();
        let north = cars
            .iter()
            .filter(|c| c.direction == Direction::North)
            .count();
        let east = cars
            .iter()
            .filter(|c| c.direction == Direction::East)
            .count();
        let counts = [south, west, north, east];

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

                let capacity = lane_capacity(self.active);
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
                self.timer = 1.5;
            }
        }
    }

    fn pick_next(&self, counts: &[usize; 4]) -> Direction {
        let current_idx = DIRECTIONS.iter().position(|&d| d == self.active).unwrap();

        let mut best_idx = (current_idx + 1) % 4;
        let mut best_count = 0;
        let mut best_wait: f64 = -1.0;

        for (i, &cnt) in counts.iter().enumerate() {
            if i == current_idx {
                continue;
            }

            let wait = self.waiting_time[i];
            let is_better = cnt > best_count || (cnt == best_count && wait > best_wait);

            if is_better {
                best_count = cnt;
                best_wait = wait;
                best_idx = i;
            }
        }

        DIRECTIONS[best_idx]
    }

    pub fn draw(&self) {
        let mv = WINDOW_WIDTH as f32 / 2.0;
        let mh = WINDOW_HEIGHT as f32 / 2.0;
        let gap = LANE_WIDTH;
        let s = TRAFFIC_LIGHT_SIZE;

        let tl = if !self.clearing && self.active == Direction::South {
            GREEN
        } else {
            RED
        };
        let tr = if !self.clearing && self.active == Direction::West {
            GREEN
        } else {
            RED
        };
        let bl = if !self.clearing && self.active == Direction::East {
            GREEN
        } else {
            RED
        };
        let br = if !self.clearing && self.active == Direction::North {
            GREEN
        } else {
            RED
        };

        draw_rectangle(
            mv - gap - 15.0 - s / 2.0,
            mh - gap - 15.0 - s / 2.0,
            s,
            s,
            tl,
        );
        draw_rectangle(
            mv + gap + 15.0 - s / 2.0,
            mh - gap - 15.0 - s / 2.0,
            s,
            s,
            tr,
        );
        draw_rectangle(
            mv - gap - 15.0 - s / 2.0,
            mh + gap + 15.0 - s / 2.0,
            s,
            s,
            bl,
        );
        draw_rectangle(
            mv + gap + 15.0 - s / 2.0,
            mh + gap + 15.0 - s / 2.0,
            s,
            s,
            br,
        );
    }
}
