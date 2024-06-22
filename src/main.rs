mod grid;

use grid::GRID;
use raylib::prelude::*;
const SPEED: f32 = 3.0;
const GRID_ROWS: i32 = 10;
const GRID_COLS: i32 = 10;
const GRID_LINE_THICK: f32 = 1.0;
const PLAYER_RADIUS: f32 = 10.0;
const EPS: f32 = 1e-4;

fn wasd(d: &RaylibDrawHandle, p: &mut Vector2) {
    if d.is_key_down(KeyboardKey::KEY_A) {
        p.x -= SPEED * d.get_frame_time();
    }
    if d.is_key_down(KeyboardKey::KEY_D) {
        p.x += SPEED * d.get_frame_time();
    }
    if d.is_key_down(KeyboardKey::KEY_W) {
        p.y -= SPEED * d.get_frame_time();
    }
    if d.is_key_down(KeyboardKey::KEY_S) {
        p.y += SPEED * d.get_frame_time();
    }
}

fn snap_step(p: Vector2, q: Vector2) -> Vector2 {
    let dir = q - p;
    let cx = if dir.x > 0.0 { p.x.ceil() } else { p.x.floor() };
    let cy = if dir.y > 0.0 { p.y.ceil() } else { p.y.floor() };

    // Handle horizontal and vertical lines
    if dir.x == 0.0 {
        return Vector2::new(p.x, cy);
    }
    if dir.y == 0.0 {
        return Vector2::new(cx, p.y);
    }

    let m = dir.y / dir.x;
    let cxv = Vector2::new(cx, m * (cx - p.x) + p.y);
    let cyv = Vector2::new((cy - p.y) / m + p.x, cy);

    let out = if (p - cxv).length_sqr() < (p - cyv).length_sqr() {
        cxv
    } else {
        cyv
    };

    // If the snapped point is further than the endpoint we are interpolating to, just return the endpoint
    if (p - out).length_sqr() > dir.length_sqr() {
        q
    } else {
        out
    }
}

// Returns whether a point is within an object placed on the grid
fn collision(p: Vector2) -> bool {
    return GRID[p.y.floor() as usize][p.x.floor() as usize];
}

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 800).title("Raycasting").build();
    rl.set_target_fps(120);

    let w = rl.get_screen_width();
    let h = rl.get_screen_height() as f32;
    let mut p1 = Vector2::new(GRID_COLS as f32 * 0.45, GRID_ROWS as f32 * 0.45);
    let grid_size = h / GRID_ROWS as f32;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::from_hex("181818").expect("Invalid color provided!"));

        wasd(&d, &mut p1);
        let p3 = d.get_mouse_position().scale_by(1.0 / grid_size);

        for y in 0..GRID_ROWS {
            let height = y as f32 * grid_size;
            d.draw_line_ex(
                Vector2::new(0.0, height),
                Vector2::new(w as f32, height),
                GRID_LINE_THICK,
                Color::GRAY,
            );
        }
        for x in 0..GRID_COLS {
            let width = x as f32 * grid_size;
            d.draw_line_ex(
                Vector2::new(width, 0.0),
                Vector2::new(width, h as f32),
                GRID_LINE_THICK,
                Color::GRAY,
            );
        }

        for y in 0..GRID_ROWS {
            for x in 0..GRID_COLS {
                if GRID[y as usize][x as usize] {
                    d.draw_rectangle(
                        x * grid_size as i32,
                        y * grid_size as i32,
                        grid_size as i32,
                        grid_size as i32,
                        Color::GRAY,
                    );
                }
            }
        }

        let mut p2 = p1;
        while p2 != p3 {
            let dir = p3 - p1;
            p2 = snap_step(p2 + dir * EPS, p3);
            d.draw_circle_v(p2.scale_by(grid_size), PLAYER_RADIUS, Color::MAGENTA);

            if collision(p2 + dir * EPS) {
                break;
            }
        }

        d.draw_line_ex(
            p1.scale_by(grid_size),
            p2.scale_by(grid_size),
            4.0,
            Color::MAGENTA,
        );
        d.draw_circle_v(p1.scale_by(grid_size), PLAYER_RADIUS, Color::MAGENTA);
        d.draw_circle_v(p3.scale_by(grid_size), PLAYER_RADIUS, Color::YELLOW);

        // d.draw_fps(0, 0);
    }
}
