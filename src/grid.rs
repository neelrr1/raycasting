use crate::{GRID_COLS, GRID_ROWS};

pub const GRID: [[Option<&str>; GRID_COLS as usize]; GRID_ROWS as usize] = [
    [None, None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None, None],
    [
        None,
        None,
        None,
        Some("redbrick.png"),
        Some("eagle.png"),
        Some("redbrick.png"),
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        Some("wood.png"),
        None,
        Some("colorstone.png"),
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
];
