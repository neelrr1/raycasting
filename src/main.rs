mod grid;

use std::cmp::max;

use grid::GRID;
use raylib::prelude::*;
const SPEED: f32 = 3.0;
const SENS: f32 = 0.005;
const GRID_ROWS: i32 = 10;
const GRID_COLS: i32 = 10;
const GRID_LINE_THICK: f32 = 1.0;
const PLAYER_RADIUS: f32 = 10.0;
const EPS: f32 = 1e-3;

fn wasd(d: &RaylibDrawHandle, p: &mut Vector2, dir: Vector2) {
    let inv_dir = Vector2::new(-dir.y, dir.x);
    if d.is_key_down(KeyboardKey::KEY_A) {
        *p += -inv_dir * SPEED * d.get_frame_time();
    }
    if d.is_key_down(KeyboardKey::KEY_D) {
        *p += inv_dir * SPEED * d.get_frame_time();
    }
    if d.is_key_down(KeyboardKey::KEY_W) {
        *p += dir * SPEED * d.get_frame_time();
    }
    if d.is_key_down(KeyboardKey::KEY_S) {
        *p += -dir * SPEED * d.get_frame_time();
    }
}

fn draw_grid(d: &mut RaylibDrawHandle, grid_size: f32, boundary: Rectangle) {
    for y in 0..GRID_ROWS {
        for x in 0..GRID_COLS {
            if let Some(c) = GRID[y as usize][x as usize] {
                d.draw_rectangle_v(
                    Vector2::new(
                        x as f32 * grid_size + boundary.x,
                        y as f32 * grid_size + boundary.y,
                    ),
                    Vector2::one().scale_by(grid_size),
                    c,
                );
            }
        }
    }

    for y in 0..((boundary.height / grid_size) as i32 + 1) {
        d.draw_line_ex(
            Vector2::new(boundary.x, boundary.y + grid_size * y as f32),
            Vector2::new(
                boundary.x + boundary.width,
                boundary.y + grid_size * y as f32,
            ),
            GRID_LINE_THICK,
            Color::GRAY,
        );
    }
    for x in 0..((boundary.width / grid_size) as i32 + 1) {
        d.draw_line_ex(
            Vector2::new(boundary.x + grid_size * x as f32, boundary.y),
            Vector2::new(
                boundary.x + grid_size * x as f32,
                boundary.y + boundary.height,
            ),
            GRID_LINE_THICK,
            Color::GRAY,
        );
    }
}

fn snap_step(p: Vector2, dir: Vector2) -> Vector2 {
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

    out
}

// Returns whether a point is within an object placed on the grid
fn collision(p: Vector2) -> Option<Color> {
    let y = p.y.floor();
    let x = p.x.floor();

    if x >= 0.0 && x < GRID[0].len() as f32 && y >= 0.0 && y < GRID.len() as f32 {
        GRID[y as usize][x as usize]
    } else {
        None
    }
}

fn on_grid(p: Vector2) -> bool {
    let y = p.y.floor();
    let x = p.x.floor();
    x >= 0.0 && x < GRID[0].len() as f32 && y >= 0.0 && y < GRID.len() as f32
}

fn find_collision(p: Vector2, dir: Vector2) -> (Vector2, Option<Color>) {
    let mut p2 = p;
    let mut i = 0;
    loop {
        p2 = snap_step(p2 + dir * EPS, dir);

        if !on_grid(p2) || i > max(GRID_ROWS, GRID_COLS) {
            return (p2, None);
        }
        if let Some(c) = collision(p2 + dir * EPS) {
            return (p2, Some(c));
        }
    }
}

fn minimap(
    d: &mut RaylibDrawHandle,
    boundary: Rectangle,
    grid_size: f32,
    player: Vector2,
    dir: Vector2,
) {
    draw_grid(d, grid_size, boundary);
    let offset = Vector2::new(boundary.x, boundary.y);

    d.draw_circle_v(
        player.scale_by(grid_size) + offset,
        PLAYER_RADIUS,
        Color::MAGENTA,
    );

    // Draw FOV
    d.draw_line_ex(
        player.scale_by(grid_size) + offset,
        (player + dir).scale_by(grid_size) + offset,
        3.0,
        Color::BLUE,
    );
    let camera_plane = Vector2::new(-dir.y, dir.x);
    d.draw_line_ex(
        (player + dir).scale_by(grid_size) + offset,
        (player + dir + camera_plane).scale_by(grid_size) + offset,
        3.0,
        Color::BLUE,
    );
    d.draw_line_ex(
        (player + dir).scale_by(grid_size) + offset,
        (player + dir - camera_plane).scale_by(grid_size) + offset,
        3.0,
        Color::BLUE,
    );
}

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 800).title("Raycasting").build();
    rl.set_target_fps(120);

    let mut p1 = Vector2::new(GRID_COLS as f32 * 0.45, GRID_ROWS as f32 * 0.45);
    let mut dir: Vector2 = Vector2::new(0.0, -1.0);
    let grid_size = 40.0;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::from_hex("181818").expect("Invalid color provided!"));

        wasd(&d, &mut p1, dir);
        // dir always has a length of 1
        dir.rotate(d.get_mouse_delta().x * SENS);

        let ortho = Vector2::new(-dir.y, dir.x);
        let camera_plane_start = p1 + dir + ortho;
        let camera_plane_end = p1 + dir - ortho;

        for x in 0..d.get_screen_width() {
            let t =
                camera_plane_end.lerp(camera_plane_start, x as f32 / d.get_screen_width() as f32);
            let (p2, collided_color) = find_collision(p1, t - p1);

            if let Some(c) = collided_color {
                let h = 1.0 / p1.distance_to(p2) * d.get_screen_height() as f32;
                d.draw_line_v(
                    Vector2::new(x as f32, (d.get_screen_height() as f32 - h / 2.0) / 2.0),
                    Vector2::new(x as f32, (d.get_screen_height() as f32 + h / 2.0) / 2.0),
                    c,
                )
            }
        }

        minimap(
            &mut d,
            Rectangle::new(5.0, 5.0, 400.0, 200.0),
            grid_size,
            p1,
            dir,
        );

        // d.draw_fps(0, 0);
    }
}
