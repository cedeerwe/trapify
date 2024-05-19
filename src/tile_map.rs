use comfy::*;

pub const TILE_SIZE: f32 = 1.; // This is in world coordinates
pub const ROWS: u32 = 5;
pub const COLUMNS: u32 = 28; // ZOOM is setup to 30, 1 margin on both sides

pub fn draw() {
    // TODO: 1/64 does not work well in general
    let draw_grid_line = |p1, p2| draw_line(p1, p2, 1. / 64., DARKGRAY, 1);

    let x_start = x_min();
    let x_end = x_max();
    for row in 0..=ROWS {
        let y = y_into_absolute_start(row);
        draw_grid_line(Vec2::new(x_start, y), Vec2::new(x_end, y))
    }

    let y_start = y_min();
    let y_end = y_max();
    for column in 0..=COLUMNS {
        let x = x_into_absolute_start(column);
        draw_grid_line(Vec2::new(x, y_start), Vec2::new(x, y_end))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileMapPos {
    pub x: u32,
    pub y: u32,
}

pub fn x_min() -> f32 {
    main_camera().screen_top_left().x + TILE_SIZE // MARGIN
}

pub fn x_max() -> f32 {
    x_min() + (COLUMNS as f32) * TILE_SIZE
}

pub fn x_from_absolute(x: f32) -> Option<u32> {
    let x = (x - x_min()).div_euclid(TILE_SIZE);
    if x >= 0. && x < COLUMNS as f32 {
        Some(x as u32)
    } else {
        None
    }
}

pub fn x_into_absolute_start(x: u32) -> f32 {
    x_min() + (x as f32) * TILE_SIZE
}

pub fn x_into_absolute_mid(x: u32) -> f32 {
    x_into_absolute_start(x) + TILE_SIZE / 2.
}

pub fn x_into_absolute_end(x: u32) -> f32 {
    x_into_absolute_start(x) + TILE_SIZE
}

pub fn y_max() -> f32 {
    main_camera().screen_top_left().y - TILE_SIZE // margin
}

pub fn y_min() -> f32 {
    y_max() - (ROWS as f32) * TILE_SIZE
}

pub fn y_from_absolute(y: f32) -> Option<u32> {
    let y = (y - y_min()).div_euclid(TILE_SIZE);
    if y >= 0. && y < ROWS as f32 {
        Some(y as u32)
    } else {
        None
    }
}

pub fn y_into_absolute_start(y: u32) -> f32 {
    y_min() + (y as f32) * TILE_SIZE
}

pub fn y_into_absolute_mid(y: u32) -> f32 {
    y_into_absolute_start(y) + TILE_SIZE / 2.
}

pub fn y_into_absolute_end(y: u32) -> f32 {
    y_into_absolute_start(y) + TILE_SIZE
}

impl TileMapPos {
    pub fn new(x: u32, y: u32) -> TileMapPos {
        Self { x, y }
    }

    pub fn from_absolute(pos: Vec2) -> Option<TileMapPos> {
        let x = x_from_absolute(pos.x)?;
        let y = y_from_absolute(pos.y)?;
        Some(TileMapPos { x, y })
    }

    pub fn into_absolute_start(self) -> Vec2 {
        Vec2 {
            x: x_into_absolute_start(self.x),
            y: y_into_absolute_start(self.y),
        }
    }

    pub fn into_absolute_end(self) -> Vec2 {
        Vec2 {
            x: x_into_absolute_end(self.x),
            y: y_into_absolute_end(self.y),
        }
    }

    pub fn into_absolute_mid(self) -> Vec2 {
        Vec2 {
            x: x_into_absolute_mid(self.x),
            y: y_into_absolute_mid(self.y),
        }
    }
}
