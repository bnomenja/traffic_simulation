use macroquad::prelude::*;

mod car;
mod consts;
mod light;
mod road;

use car::{CarManager, Direction};
use consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use light::TrafficController;
use road::draw_roads;

fn window_conf() -> Conf {
    Conf {
        window_title: "Traffic Control Simulation".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut manager = CarManager::new();
    let mut lights = TrafficController::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // ── Spawn cars ──
        if is_key_pressed(KeyCode::Up) {
            manager.try_spawn_car(Direction::North);
        }
        if is_key_pressed(KeyCode::Down) {
            manager.try_spawn_car(Direction::South);
        }
        if is_key_pressed(KeyCode::Right) {
            manager.try_spawn_car(Direction::East);
        }
        if is_key_pressed(KeyCode::Left) {
            manager.try_spawn_car(Direction::West);
        }
        if is_key_pressed(KeyCode::R) {
            manager.try_spawn_car(Direction::random());
        }

        // Clear all cars
        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Backspace) {
            manager.cars.clear();
        }

        // ── Update ──
        let dt = get_frame_time();
        lights.update(dt as f64, &manager.cars);
        manager.update(dt, &lights);

        // ── Draw ──
        clear_background(Color::from_rgba(4, 96, 85, 255));
        draw_roads();
        lights.draw();
        manager.draw();

        next_frame().await;
    }
}
