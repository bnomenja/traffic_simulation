use macroquad::prelude::*;
use crate::consts::*;

pub fn draw_roads() {
    let w = WINDOW_WIDTH as f32;
    let h = WINDOW_HEIGHT as f32;
    let g = LANE_WIDTH;
    let cx = w / 2.0;
    let cy = h / 2.0;

    // Road surface
    let road_color = Color::from_rgba(30, 30, 30, 255);
    draw_rectangle(cx - g, 0.0, g * 2.0, h, road_color);
    draw_rectangle(0.0, cy - g, w, g * 2.0, road_color);

    let dash = 20.0;
    let space = 15.0;
    let thickness = 2.0;

    // Horizontal lane markings
    draw_dashed_line(vec2(0.0, cy - g), vec2(w, cy - g), dash, space, thickness, WHITE);
    draw_dashed_line(vec2(0.0, cy), vec2(w, cy), dash, space, thickness, YELLOW);
    draw_dashed_line(vec2(0.0, cy + g), vec2(w, cy + g), dash, space, thickness, WHITE);

    // Vertical lane markings
    draw_dashed_line(vec2(cx - g, 0.0), vec2(cx - g, h), dash, space, thickness, WHITE);
    draw_dashed_line(vec2(cx, 0.0), vec2(cx, h), dash, space, thickness, YELLOW);
    draw_dashed_line(vec2(cx + g, 0.0), vec2(cx + g, h), dash, space, thickness, WHITE);
}

fn draw_dashed_line(
    start: Vec2,
    end: Vec2,
    dash_len: f32,
    gap_len: f32,
    thickness: f32,
    color: Color,
) {
    let dir = end - start;
    let len = dir.length();
    if len <= 0.0 {
        return;
    }
    let step = dir / len;
    let mut travelled = 0.0;
    while travelled < len {
        let a = start + step * travelled;
        let b = start + step * (travelled + dash_len).min(len);
        draw_line(a.x, a.y, b.x, b.y, thickness, color);
        travelled += dash_len + gap_len;
    }
}
