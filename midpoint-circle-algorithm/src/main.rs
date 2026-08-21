// A circle with  radius R centered at (0, 0) followes X^2 + y^2 = R^2

use macroquad::prelude::*;

// functions:
// draw_pixel();
// draw_rectangle() from marcroquad::prelude::draw_rectangle;
// plot_8_octants()
// draw_midpoint_circle()
// clear_background() to add bg color
// next_frame() to gen next frame


// Func: Plot a pixel at custom offset (cx, cy)
fn draw_pixel(x: i32, y: i32, color: Color) {
    // Macroquad uses (f32, f32) screen coordinates, so cast i32 to f32.
    // 1.0x1.0 rectangle acts as a single pixel.
    draw_rectangle(x as f32, y as f32, 1.0, 1.0, color);
}

// Reflection func: 8-Fold Symmetry
fn plot_8_octants(cx: i32, cy: i32, x: i32, y: i32, color: Color){
    draw_pixel(cx + x, cy + y, color);
    draw_pixel(cx - x, cy + y, color);
    draw_pixel(cx + x, cy - y, color);
    draw_pixel(cx - x, cy - y, color);
    draw_pixel(cx + y, cy + x, color);
    draw_pixel(cx - y, cy + x, color);
    draw_pixel(cx + y, cy - x, color);
    draw_pixel(cx - y, cy - x, color);
}

// core algorithm: mid-point circle
fn draw_midpoint_circle(cx: i32, cy: i32, radius: i32, color: Color) {
    let mut x = 0;
    let mut y = radius;
    let mut p = 1 - radius; // initial decision parameter

    // plot initial point on top octant boundary
    plot_8_octants(cx, cy, x, y, color);

    while x < y {
        x += 1;

        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }

        plot_8_octants(cx, cy, x, y, color);
    }
}

// main
#[macroquad::main("Mid-Point Circle Generator")]
async fn main() {
    loop {
        // clear background to dark gray
        clear_background(DARKGRAY);
        
        // center coordinates of the window
        let center_x = (screen_width() / 2.0) as i32;
        let center_y = (screen_height() / 2.0) as i32;
        let radius = 150;

        // calling algorithm
        draw_midpoint_circle(center_x, center_y, radius, YELLOW);

        // render frame to screen

        next_frame().await;
    }
}