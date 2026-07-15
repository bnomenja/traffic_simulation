use macroquad::prelude::*;

mod consts;
mod road;
mod car;
mod light;

use consts::{WINDOW_WIDTH, WINDOW_HEIGHT};
use road::draw_roads;
use car::{CarManager, Direction};
use light::TrafficController;

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
    //let mut manager = CarManager::new();
    //let mut lights = TrafficController::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        /*if is_key_pressed(KeyCode::Up) {
            manager.try_spawn_car(Direction::South);
        }

        if is_key_pressed(KeyCode::Down) {
            manager.try_spawn_car(Direction::North);
        }

        if is_key_pressed(KeyCode::Right) {
            manager.try_spawn_car(Direction::West);
        }

        if is_key_pressed(KeyCode::Left) {
            manager.try_spawn_car(Direction::East);
        }

        if is_key_pressed(KeyCode::R) {
            manager.try_spawn_car(Direction::random());
        }*/

        //let t = get_frame_time();
        //lights.update(t as f64, &manager.cars);
        //manager.update(t, &lights);

        draw_roads();
        //lights.draw();
        //manager.draw();

        next_frame().await;
    }
}