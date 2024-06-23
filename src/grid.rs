use raylib::color::Color;

use crate::{GRID_COLS, GRID_ROWS};

pub const GRID: [[Option<Color>; GRID_COLS as usize]; GRID_ROWS as usize] = [
    [None, None, None, None, None, None, None, None, None, None],
    [
        None,
        None,
        None,
        Some(Color::YELLOW),
        Some(Color::MAGENTA),
        Some(Color::GREEN),
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        Some(Color::RED),
        None,
        Some(Color::BLUE),
        None,
        None,
        None,
        None,
    ],
    [None, None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None, None],
];
