mod grid;

use std::f32::consts::PI;

use grid::GRID;
use raylib::prelude::*;
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 800;
const TARGET_FPS: u32 = 120;
const SPEED: f32 = 3.0;
const SENS: f32 = 0.005;
const GRID_ROWS: i32 = 10;
const GRID_COLS: i32 = 10;
const GRID_LINE_THICK: f32 = 1.0;
const GRID_SIZE: f32 = 20.0;
const PLAYER_RADIUS: f32 = 10.0;
// setting too low can cause performance issues due to floating point math
const EPS: f32 = 1e-4;
const FOV: f32 = 90.0;

const MINIMAP_PADDING: f32 = 5.0;
const MINIMAP_SIZE: f32 = 200.0;

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

fn draw_grid(d: &mut RaylibDrawHandle, boundary: Rectangle) {
    for y in 0..GRID_ROWS {
        for x in 0..GRID_COLS {
            if let Some(c) = GRID[y as usize][x as usize] {
                d.draw_rectangle_v(
                    Vector2::new(
                        x as f32 * GRID_SIZE + boundary.x,
                        y as f32 * GRID_SIZE + boundary.y,
                    ),
                    Vector2::one().scale_by(GRID_SIZE),
                    c,
                );
            }
        }
    }

    for y in 0..((boundary.height / GRID_SIZE) as i32 + 1) {
        d.draw_line_ex(
            Vector2::new(boundary.x, boundary.y + GRID_SIZE * y as f32),
            Vector2::new(
                boundary.x + boundary.width,
                boundary.y + GRID_SIZE * y as f32,
            ),
            GRID_LINE_THICK,
            Color::GRAY,
        );
    }
    for x in 0..((boundary.width / GRID_SIZE) as i32 + 1) {
        d.draw_line_ex(
            Vector2::new(boundary.x + GRID_SIZE * x as f32, boundary.y),
            Vector2::new(
                boundary.x + GRID_SIZE * x as f32,
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
    loop {
        p2 = snap_step(p2 + dir * EPS, dir);

        if !on_grid(p2) {
            return (p2, None);
        }
        if let Some(c) = collision(p2 + dir * EPS) {
            return (p2, Some(c));
        }
    }
}

fn dir_to_camera_plane(dir: Vector2) -> Vector2 {
    Vector2::new(-dir.y, dir.x) * (0.5 / (FOV / 2.0 / 360.0 * 2.0 * PI).tan())
}

fn minimap(d: &mut RaylibDrawHandle, boundary: Rectangle, player: Vector2, dir: Vector2) {
    draw_grid(d, boundary);
    let offset = Vector2::new(boundary.x, boundary.y);

    d.draw_circle_v(
        player.scale_by(GRID_SIZE) + offset,
        PLAYER_RADIUS,
        Color::MAGENTA,
    );

    // Draw FOV
    d.draw_line_ex(
        player.scale_by(GRID_SIZE) + offset,
        (player + dir).scale_by(GRID_SIZE) + offset,
        3.0,
        Color::BLUE,
    );
    let camera_plane = dir_to_camera_plane(dir);
    d.draw_line_ex(
        (player + dir).scale_by(GRID_SIZE) + offset,
        (player + dir + camera_plane).scale_by(GRID_SIZE) + offset,
        3.0,
        Color::BLUE,
    );
    d.draw_line_ex(
        (player + dir).scale_by(GRID_SIZE) + offset,
        (player + dir - camera_plane).scale_by(GRID_SIZE) + offset,
        3.0,
        Color::BLUE,
    );
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycasting")
        .build();
    rl.set_target_fps(TARGET_FPS);

    let mut p1 = Vector2::new(GRID_COLS as f32 * 0.45, GRID_ROWS as f32 * 0.75);
    let mut dir: Vector2 = Vector2::new(0.0, -1.0);
    rl.get_mouse_delta();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::from_hex("181818").expect("Invalid color provided!"));

        wasd(&d, &mut p1, dir);
        // dir is always normalized (has a length of 1)
        dir.rotate(d.get_mouse_delta().x * SENS);

        // since this vector is just half of the camera plane, make it length 0.5
        let ortho = dir_to_camera_plane(dir);
        let camera_plane_start = p1 + dir - ortho;
        let camera_plane_end = p1 + dir + ortho;

        for x in 0..d.get_screen_width() {
            let t =
                camera_plane_start.lerp(camera_plane_end, x as f32 / d.get_screen_width() as f32);
            let (p2, collided_color) = find_collision(p1, t - p1);

            if let Some(c) = collided_color {
                let mut h = 1.0 / (p2 - p1).dot(dir);
                h *= d.get_screen_height() as f32;
                d.draw_line_v(
                    Vector2::new(x as f32, (d.get_screen_height() as f32 - h) / 2.0),
                    Vector2::new(x as f32, (d.get_screen_height() as f32 + h) / 2.0),
                    c,
                )
            }
        }

        minimap(
            &mut d,
            Rectangle::new(
                SCREEN_WIDTH as f32 - MINIMAP_SIZE - MINIMAP_PADDING,
                MINIMAP_PADDING,
                MINIMAP_SIZE,
                MINIMAP_SIZE,
            ),
            p1,
            dir,
        );

        d.draw_fps(0, 0);
    }
}
/* TODO:
 * Textures!
 * Add shading
*/
