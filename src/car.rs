use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::consts::*;
use crate::light::TrafficController;

/// Holds the loaded car sprites and maps each car color to the right texture.
pub struct CarTextures {
    yellow: Texture2D,
    blue: Texture2D,
    red: Texture2D,
}

impl CarTextures {
    /// Loads every sprite up front. Call once at startup, before the main loop,
    /// since `load_texture` is async.
    pub async fn load() -> Self {
        let yellow = load_texture("assets/car_yellow.png").await.unwrap();
        let blue = load_texture("assets/car_blue.png").await.unwrap();
        let red = load_texture("assets/car_red.png").await.unwrap();

        Self { yellow, blue, red }
    }

    fn for_color(&self, color: Color) -> &Texture2D {
        if color == YELLOW {
            &self.yellow
        } else if color == BLUE {
            &self.blue
        } else {
            &self.red
        }
    }
}

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

    pub fn draw(&self, textures: &CarTextures) {
        let texture = textures.for_color(self.color);
        let (dx, dy) = self.heading();

        let rotation = dx.atan2(-dy);

        let half = CAR_WIDTH / 2.0;
        draw_texture_ex(
            texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(CAR_WIDTH, CAR_WIDTH)),
                rotation,
                pivot: Some(vec2(self.x + half, self.y + half)),
                ..Default::default()
            },
        );
    }

    // heading describes how x and y are growing to know the lane he is curently heading
    // (0.0, -1.0)-> heading North; (0.0, 1.0)-> heading South; (1.0, 0.0)-> heading East; (-1.0, 0.0)-> heading West; 
    pub fn heading(&self) -> (f32, f32) {
        let mv = WINDOW_WIDTH as f32 / 2.0;
        let mh = WINDOW_HEIGHT as f32 / 2.0;

        match self.direction {
            Direction::North => {
                if self.y >= mh + 12.0 && self.y <= mh + 18.0 && self.color == RED {
                    (1.0, 0.0) // turned right -> now heading East
                } else if self.y >= mh - 48.0 && self.y <= mh - 43.0 && self.color == YELLOW {
                    (-1.0, 0.0) // turned left -> now heading West
                } else {
                    (0.0, -1.0) // still straight -> heading North
                }
            }
            Direction::South => {
                if self.y >= mh - 48.0 && self.y <= mh - 43.0 && self.color == RED {
                    (-1.0, 0.0) // turned right -> now heading West
                } else if self.y >= mh + 12.0 && self.y <= mh + 18.0 && self.color == YELLOW {
                    (1.0, 0.0) // turned left -> now heading East
                } else {
                    (0.0, 1.0) // still straight -> heading South
                }
            }
            Direction::East => {
                if self.x >= mv - 48.0 && self.x <= mv - 42.0 && self.color == RED {
                    (0.0, 1.0) // turned right -> now heading South
                } else if self.x >= mv + 12.0 && self.x <= mv + 18.0 && self.color == YELLOW {
                    (0.0, -1.0) // turned left -> now heading North
                } else {
                    (1.0, 0.0) // still straight -> heading East
                }
            }
            Direction::West => {
                if self.x >= mv + 12.0 && self.x <= mv + 18.0 && self.color == RED {
                    (0.0, -1.0) // turned right -> now heading North
                } else if self.x >= mv - 48.0 && self.x <= mv - 42.0 && self.color == YELLOW {
                    (0.0, 1.0) // turned left -> now heading South
                } else {
                    (-1.0, 0.0) // still straight -> heading West
                }
            }
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
            let my_heading = self.cars[i].heading(); 

            for j in 0..self.cars.len() {
                if i == j {
                    continue;
                }

                let other = &self.cars[j];

                if other.heading() != my_heading {
                    continue;
                }

                let dx = other.x - mx;
                let dy = other.y - my;

                let forward = dx * my_heading.0 + dy * my_heading.1; //scalar product of 2 vector

                if forward > 0.0 && forward < safety_gap  { // ahead of me && too close && same lane
                    blocked = true;
                    break;
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

    pub fn draw(&self, textures: &CarTextures) {
        for car in &self.cars {
            car.draw(textures);
        }
    }
}