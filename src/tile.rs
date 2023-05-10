use serde::{Deserialize, Serialize};

use crate::board::BOARD_SIZE;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tile {
    red: u8,
    green: u8,
    blue: u8,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn set_color(&mut self, (red, green, blue): (u8, u8, u8)) {
        self.red = red;
        self.green = green;
        self.blue = blue;
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TileReq {
    row: usize,
    col: usize,
    tile: Tile,
}

impl TileReq {
    pub fn is_valid(&self) -> bool {
        self.col < BOARD_SIZE && self.row < BOARD_SIZE
    }
    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_col(&self) -> usize {
        self.col
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        (self.tile.red, self.tile.green, self.tile.blue)
    }
}
