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
            Direction::East => (10.0, mh + 15.0),
            Direction::West => (WINDOW_WIDTH as f32 - 35.0, mh - 45.0),
        };

        // Random color determines turn: RED=right, YELLOW=left, BLUE=straight
        let color = match gen_range(0, 3) {
            0 => YELLOW,
            1 => BLUE,
            _ => RED,
        };

        Self { x, y, direction, color }
    }

    pub fn update(&mut self, dt: f32, is_green: bool, ahead_progress: Option<f32>) {
        // Stop if blocked by a car ahead
        if let Some(dist) = ahead_progress {
            if dist < SAFETY_GAP + CAR_WIDTH {
                return;
            }
        }

        let speed = VEHICLE_SPEED;
        let mv = WINDOW_WIDTH as f32 / 2.0;
        let mh = WINDOW_HEIGHT as f32 / 2.0;

        match self.direction {
            Direction::North => {
                // Right-turn zone (RED car turns East)
                if self.y >= mh + 12.0 && self.y <= mh + 18.0 && self.color == RED {
                    self.y = mh + 15.0;
                    self.x += speed * dt;
                }
                // Left-turn zone (YELLOW car turns West)
                else if self.y >= mh - 48.0 && self.y <= mh - 43.0 && self.color == YELLOW {
                    self.y = mh - 45.0;
                    self.x -= speed * dt;
                }
                // Normal straight movement
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
                // Right-turn zone (RED car turns West)
                if self.y >= mh - 48.0 && self.y <= mh - 43.0 && self.color == RED {
                    self.y = mh - 45.0;
                    self.x -= speed * dt;
                }
                // Left-turn zone (YELLOW car turns East)
                else if self.y >= mh + 12.0 && self.y <= mh + 18.0 && self.color == YELLOW {
                    self.y = mh + 15.0;
                    self.x += speed * dt;
                }
                // Normal straight movement
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
                // Right-turn zone (RED car turns South)
                if self.x >= mv - 48.0 && self.x <= mv - 42.0 && self.color == RED {
                    self.x = mv - 45.0;
                    self.y += speed * dt;
                }
                // Left-turn zone (YELLOW car turns North)
                else if self.x >= mv + 12.0 && self.x <= mv + 18.0 && self.color == YELLOW {
                    self.x = mv + 15.0;
                    self.y -= speed * dt;
                }
                // Normal straight movement
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
                // Right-turn zone (RED car turns North)
                if self.x >= mv + 12.0 && self.x <= mv + 18.0 && self.color == RED {
                    self.x = mv + 15.0;
                    self.y -= speed * dt;
                }
                // Left-turn zone (YELLOW car turns South)
                else if self.x >= mv - 48.0 && self.x <= mv - 42.0 && self.color == YELLOW {
                    self.x = mv - 45.0;
                    self.y += speed * dt;
                }
                // Normal straight movement
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

/// Returns the *effective* movement direction of a car, taking turn zones into account.
fn effective_move(dir: Direction, color: Color, x: f32, y: f32, mv: f32, mh: f32) -> MoveDir {
    match dir {
        Direction::North => {
            if (y >= mh + 12.0 && y <= mh + 18.0) || y == mh + 15.0 {
                if color == RED {
                    return MoveDir::Right;
                }
            }
            if (y >= mh - 48.0 && y <= mh - 43.0) || y == mh - 45.0 {
                if color == YELLOW {
                    return MoveDir::Left;
                }
            }
            MoveDir::Up
        }
        Direction::South => {
            if (y >= mh - 48.0 && y <= mh - 43.0) || y == mh - 45.0 {
                if color == RED {
                    return MoveDir::Left;
                }
            }
            if (y >= mh + 12.0 && y <= mh + 18.0) || y == mh + 15.0 {
                if color == YELLOW {
                    return MoveDir::Right;
                }
            }
            MoveDir::Down
        }
        Direction::East => {
            if (x >= mv - 48.0 && x <= mv - 42.0) || x == mv - 45.0 {
                if color == RED {
                    return MoveDir::Down;
                }
            }
            if (x >= mv + 12.0 && x <= mv + 18.0) || x == mv + 15.0 {
                if color == YELLOW {
                    return MoveDir::Up;
                }
            }
            MoveDir::Right
        }
        Direction::West => {
            if (x >= mv + 12.0 && x <= mv + 18.0) || x == mv + 15.0 {
                if color == RED {
                    return MoveDir::Up;
                }
            }
            if (x >= mv - 48.0 && x <= mv - 42.0) || x == mv - 45.0 {
                if color == YELLOW {
                    return MoveDir::Down;
                }
            }
            MoveDir::Left
        }
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
            Direction::East => 2,
            Direction::West => 3,
        }
    }

    pub fn try_spawn_car(&mut self, direction: Direction) {
        let now = get_time();
        let idx = Self::dir_index(direction);

        // Cooldown check
        if now - self.last_spawn[idx] < SPAWN_COOLDOWN {
            return;
        }

        let mv = WINDOW_WIDTH as f32 / 2.0;
        let mh = WINDOW_HEIGHT as f32 / 2.0;
        let (sx, sy) = match direction {
            Direction::North => (mv + 15.0, WINDOW_HEIGHT as f32 - 35.0),
            Direction::South => (mv - 45.0, 10.0),
            Direction::East => (10.0, mh + 15.0),
            Direction::West => (WINDOW_WIDTH as f32 - 35.0, mh - 45.0),
        };

        // Distance check — don't spawn on top of another car
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

    pub fn update(&mut self, t: f32, lights: &TrafficController) {
        let min_gap = CAR_WIDTH + SAFETY_GAP;
        let mv = WINDOW_WIDTH as f32 / 2.0;
        let mh = WINDOW_HEIGHT as f32 / 2.0;

        for i in 0..self.cars.len() {
            let mut ahead_dist: Option<f32> = None;
            let (mx, my) = (self.cars[i].x, self.cars[i].y);
            let my_dir = self.cars[i].direction;
            let my_color = self.cars[i].color;

            let moving = effective_move(my_dir, my_color, mx, my, mv, mh);

            for j in 0..self.cars.len() {
                if i == j {
                    continue;
                }
                if self.cars[j].direction != my_dir {
                    continue;
                }

                let (ox, oy) = (self.cars[j].x, self.cars[j].y);
                let dx = mx - ox;
                let dy = my - oy;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < min_gap {
                    let is_ahead = match moving {
                        MoveDir::Up => oy < my,
                        MoveDir::Down => oy > my,
                        MoveDir::Right => ox > mx,
                        MoveDir::Left => ox < mx,
                    };
                    if is_ahead {
                        ahead_dist = Some(dist);
                        break;
                    }
                }
            }

            let is_green = lights.is_green(my_dir);
            self.cars[i].update(t, is_green, ahead_dist);
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
