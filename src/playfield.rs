use core::fmt;

use crate::tiles::{Tetromino, Tile};

pub struct Playfield {
    pub field: [Tile; Self::ROWS * Self::COLS],
}

#[derive(Debug)]
pub struct Position(pub usize, pub usize);

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position(value.0, value.1)
    }
}

impl fmt::Debug for Playfield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &c) in self.field.iter().enumerate() {
            write!(f, "{:?}", c)?;
            if (i + 1) % Self::COLS == 0 && i < Self::COLS * Self::ROWS {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Default for Playfield {
    fn default() -> Self {
        Playfield {
            field: [Tile::Empty; Playfield::COLS * Playfield::ROWS],
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position(Playfield::COLS / 2 - 2, 0)
    }
}

impl Playfield {
    pub const COLS: usize = 20;
    pub const ROWS: usize = 10;

    pub fn add(&mut self, tetromino: &Tetromino, position: &Position) {
        let tiles: [Tile; 16] = tetromino.grid().to_owned().into(); // Probably change From<> to
                                                                    // reference. no need to do
                                                                    // this by value
        if self.is_valid_position(&position) {
            let (x, y) = (position.0, position.1);
            tiles.chunks(4).enumerate().for_each(|(i, chunk)| {
                let new_x = (y + i) * Self::COLS + x;
                self.field[new_x..new_x + 4].copy_from_slice(chunk);
            });
        }
    }
    pub fn is_valid_position(&self, position: &Position) -> bool {
        // What happens at the edge?
        position.0 <= Self::ROWS && position.1 <= Self::COLS
    }
}
