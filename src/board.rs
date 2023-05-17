use serde::Serialize;

use crate::tile::*;

#[derive(Debug, Clone, Serialize)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
}

pub const BOARD_SIZE: usize = 10;

impl Board {
    pub fn new() -> Self {
        let tiles: Vec<Vec<Tile>> = vec![vec![Tile::new(); BOARD_SIZE]; BOARD_SIZE];
        Board { tiles }
    }

    pub fn set_tile(&mut self, req: &TileReq) {
        let row = req.get_row();
        let col = req.get_col();

        let tile = self
            .tiles
            .get_mut(row)
            .expect("Failed to get row")
            .get_mut(col)
            .expect("Failed to get col");
        tile.set_color(req.get_color());
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
