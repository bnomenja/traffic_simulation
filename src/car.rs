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

impl Direction {
    pub fn random() -> Self {
        match gen_range(0, 4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Car {
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub color: Color,
}

impl Car {
    pub fn new(direction: Direction) -> Self {
        let mv = WINDOW_WIDTH as f32 / 2.0;
        let mh = WINDOW_HEIGHT as f32 / 2.0;
        let (x, y) = match direction {
            Direction::North => (mv + 15.0, WINDOW_HEIGHT as f32 - 35.0),
            Direction::South => (mv - 45.0, 10.0),
            Direction::East  => (10.0, mh + 15.0),
            Direction::West  => (WINDOW_WIDTH as f32 - 35.0, mh - 45.0),
        };

        let color = match gen_range(0, 3) {
            0 => YELLOW,
            1 => BLUE,
            _ => RED,
        };

        Self { x, y, direction, color }
    }

 
    pub fn update(&mut self, dt: f32, is_green: bool, blocked: bool) {
        if blocked {
            return;
        }

        let speed = VEHICLE_SPEED;
        let mv = WINDOW_WIDTH as f32 / 2.0;
        let mh = WINDOW_HEIGHT as f32 / 2.0;

        match self.direction {
            Direction::North => {
                if self.y >= mh + 12.0 && self.y <= mh + 18.0 && self.color == RED {
                    self.y = mh + 15.0;
                    self.x += speed * dt;
                }
                else if self.y >= mh - 48.0 && self.y <= mh - 43.0 && self.color == YELLOW {
                    self.y = mh - 45.0;
                    self.x -= speed * dt;
                }
                // Straight
                else {
                    let stop_line = mh + 65.0;
                    let clear_line = mh + 60.0;
                    if is_green {
                        self.y -= speed * dt;
                    } else if self.y >= stop_line {
                        self.y = (self.y - speed * dt).max(stop_line);
                    } else if self.y < clear_line {
                        self.y -= speed * dt;
                    }
                }
            }
            Direction::South => {
                // RED turns right (West)
                if self.y >= mh - 48.0 && self.y <= mh - 43.0 && self.color == RED {
                    self.y = mh - 45.0;
                    self.x -= speed * dt;
                }
                else if self.y >= mh + 12.0 && self.y <= mh + 18.0 && self.color == YELLOW {
                    self.y = mh + 15.0;
                    self.x += speed * dt;
                }
                else {
                    let stop_line = mh - 95.0;
                    let clear_line = mh - 90.0;
                    if is_green {
                        self.y += speed * dt;
                    } else if self.y <= stop_line {
                        self.y = (self.y + speed * dt).min(stop_line);
                    } else if self.y > clear_line {
                        self.y += speed * dt;
                    }
                }
            }
            Direction::East => {
                if self.x >= mv - 48.0 && self.x <= mv - 42.0 && self.color == RED {
                    self.x = mv - 45.0;
                    self.y += speed * dt;
                }
                else if self.x >= mv + 12.0 && self.x <= mv + 18.0 && self.color == YELLOW {
                    self.x = mv + 15.0;
                    self.y -= speed * dt;
                }
                else {
                    let stop_line = mv - 95.0;
                    let clear_line = mv - 90.0;
                    if is_green {
                        self.x += speed * dt;
                    } else if self.x <= stop_line {
                        self.x = (self.x + speed * dt).min(stop_line);
                    } else if self.x > clear_line {
                        self.x += speed * dt;
                    }
                }
            }
            Direction::West => {
                if self.x >= mv + 12.0 && self.x <= mv + 18.0 && self.color == RED {
                    self.x = mv + 15.0;
                    self.y -= speed * dt;
                }
                else if self.x >= mv - 48.0 && self.x <= mv - 42.0 && self.color == YELLOW {
                    self.x = mv - 45.0;
                    self.y += speed * dt;
                }
                
                else {
                    let stop_line = mv + 65.0;
                    let clear_line = mv + 60.0;
                    if is_green {
                        self.x -= speed * dt;
                    } else if self.x >= stop_line {
                        self.x = (self.x - speed * dt).max(stop_line);
                    } else if self.x < clear_line {
                        self.x -= speed * dt;
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, CAR_WIDTH, CAR_WIDTH, self.color);
    }

}

pub struct CarManager {

    pub cars: Vec<Car>,
    last_spawn: [f64; 4],
}

impl CarManager {

    pub fn new() -> Self {
        Self {
            cars: Vec::new(),
            last_spawn: [0.0; 4],
        }
    }

    fn dir_index(d: Direction) -> usize {
        match d {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East  => 2,
            Direction::West  => 3,
        }
    }

    pub fn try_spawn_car(&mut self, direction: Direction) {
        let now = get_time();
        let idx = Self::dir_index(direction);

        if now - self.last_spawn[idx] < SPAWN_COOLDOWN {
            return;
        }

        let mv = WINDOW_WIDTH as f32 / 2.0;
        let mh = WINDOW_HEIGHT as f32 / 2.0;
        let (sx, sy) = match direction {
            Direction::North => (mv + 15.0, WINDOW_HEIGHT as f32 - 35.0),
            Direction::South => (mv - 45.0, 10.0),
            Direction::East  => (10.0, mh + 15.0),
            Direction::West  => (WINDOW_WIDTH as f32 - 35.0, mh - 45.0),
        };

        let safe_dist = CAR_WIDTH + SAFETY_GAP;
        for car in &self.cars {
            if car.direction == direction {
                let dx = car.x - sx;
                let dy = car.y - sy;
                if (dx * dx + dy * dy).sqrt() < safe_dist {
                    return;
                }
            }
        }

        self.last_spawn[idx] = now;
        self.cars.push(Car::new(direction));
    }

    pub fn update(&mut self, dt: f32, lights: &TrafficController) {
        let safety_gap = CAR_WIDTH + SAFETY_GAP;

        for i in 0..self.cars.len() {
            let mut blocked = false;
            let (mx, my) = (self.cars[i].x, self.cars[i].y);
            let my_dir = self.cars[i].direction;

            for j in 0..self.cars.len() {
                if i == j {
                    continue;
                }
                let other = &self.cars[j];
                if other.direction != my_dir {
                    continue;
                }

                let (ox, oy) = (other.x, other.y);
                let dx = mx - ox;
                let dy = my - oy;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < safety_gap {
                    
                    let is_ahead = match my_dir {
                        Direction::North => oy < my,
                        Direction::South => oy > my,
                        Direction::East  => ox > mx,
                        Direction::West  => ox < mx,
                    };

                    if is_ahead {
                        blocked = true;
                        break;
                    }
                }
            }

            let is_green = lights.is_green(my_dir);
            self.cars[i].update(dt, is_green, blocked);
        }

        self.cars.retain(|car| {
            car.x > -CAR_WIDTH
                && car.x < WINDOW_WIDTH as f32 + CAR_WIDTH
                && car.y > -CAR_WIDTH
                && car.y < WINDOW_HEIGHT as f32 + CAR_WIDTH
        });
    }

    pub fn draw(&self) {
        for car in &self.cars {
            car.draw();
        }
    }
}
