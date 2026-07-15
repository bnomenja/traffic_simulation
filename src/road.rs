use crate::consts::*;
use macroquad::prelude::*;

pub fn draw_roads() {
    // Macroquad drawing functions use f32 values for positions and sizes.
    // Our window constants are i32, so we convert them before drawing.
    let window_width = WINDOW_WIDTH as f32;
    let window_height = WINDOW_HEIGHT as f32;

    // The center of the window is our reference point for the whole road layout.
    // With a 900x900 window, center_x and center_y are both 450.0.
    let center_x = window_width / 2.0;
    let center_y = window_height / 2.0;

    // Each road has two lanes: one lane for each direction.
    // If LANE_WIDTH is 60.0, the full road width becomes 120.0.
    let road_width = LANE_WIDTH * 2.0;
    let half_road = road_width / 2.0;

    // Macroquad rectangles start from the top-left corner.
    // To center the road, we start drawing half of the road before the center.
    let vertical_x = center_x - half_road;
    let horizontal_y = center_y - half_road;

    let asphalt = Color::from_rgba(46, 49, 55, 255);
    let intersection = Color::from_rgba(40, 43, 48, 255);
    let border = Color::from_rgba(26, 29, 33, 255);
    let lane_mark = Color::from_rgba(220, 220, 210, 255);
    let stop_line = Color::from_rgba(245, 245, 235, 255);
    let grass = Color::from_rgba(44, 100, 64, 255);

    // Draw from background to foreground.
    // Anything drawn later appears on top of what was drawn before.
    clear_background(grass);

    // Vertical road: x, y, width, height, color.
    draw_rectangle(vertical_x, 0.0, road_width, window_height, asphalt);

    // Horizontal road.
    draw_rectangle(0.0, horizontal_y, window_width, road_width, asphalt);

    // The center square is where the two roads cross.
    draw_rectangle(
        vertical_x,
        horizontal_y,
        road_width,
        road_width,
        intersection,
    );

    // Road borders help us see exactly where the road starts and ends.
    draw_line(vertical_x, 0.0, vertical_x, window_height, 4.0, border);

    draw_line(
        center_x + half_road,
        0.0,
        center_x + half_road,
        window_height,
        4.0,
        border,
    );

    draw_line(0.0, horizontal_y, window_width, horizontal_y, 4.0, border);

    draw_line(
        0.0,
        center_y + half_road,
        window_width,
        center_y + half_road,
        4.0,
        border,
    );

    // Lane marks are split into four parts.
    // This keeps dashed lines out of the middle of the intersection.
    draw_vertical_lane_marks(center_x, 0.0, center_y - half_road, lane_mark);

    draw_vertical_lane_marks(center_x, center_y + half_road, window_height, lane_mark);

    draw_horizontal_lane_marks(center_y, 0.0, center_x - half_road, lane_mark);

    draw_horizontal_lane_marks(center_y, center_x + half_road, window_width, lane_mark);

    let stop_offset = 3.0;

    let stop_thickness = 5.0;

    // Stop lines are drawn just before the intersection.
    // Later, cars can use this same visual idea as the place where they stop.
    draw_line(
        vertical_x,
        center_y - half_road - stop_offset,
        center_x,
        center_y - half_road - stop_offset,
        stop_thickness,
        stop_line,
    );

    draw_line(
        center_x,
        center_y + half_road + stop_offset,
        center_x + half_road,
        center_y + half_road + stop_offset,
        stop_thickness,
        stop_line,
    );

    draw_line(
        center_x - half_road - stop_offset,
        center_y,
        center_x - half_road - stop_offset,
        center_y + half_road,
        stop_thickness,
        stop_line,
    );

    draw_line(
        center_x + half_road + stop_offset,
        horizontal_y,
        center_x + half_road + stop_offset,
        center_y,
        stop_thickness,
        stop_line,
    );
}

// Draw dashed lane marks on the vertical road.
// This function is private because only this file needs it.
fn draw_vertical_lane_marks(x: f32, start_y: f32, end_y: f32, color: Color) {
    let dash_length = 28.0;
    let gap_length = 22.0;
    let thickness = 3.0;

    // y is mutable because we move downward after drawing each dash.
    let mut y = start_y;

    while y < end_y {
        // min() keeps the last dash from drawing past the road section.
        let dash_end = (y + dash_length).min(end_y);
        draw_line(x, y, x, dash_end, thickness, color);
        y += dash_length + gap_length;
    }
}

// Same dash logic as above, but moving from left to right on the horizontal road.
fn draw_horizontal_lane_marks(y: f32, start_x: f32, end_x: f32, color: Color) {
    let dash_length = 28.0;
    let gap_length = 22.0;
    let thickness = 3.0;

    // x is mutable because we move to the right after drawing each dash.
    let mut x = start_x;

    while x < end_x {
        // min() keeps the last dash inside the requested road section.
        let dash_end = (x + dash_length).min(end_x);
        draw_line(x, y, dash_end, y, thickness, color);
        x += dash_length + gap_length;
    }
}
