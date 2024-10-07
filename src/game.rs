use std::borrow::BorrowMut;

use crate::{
    playfield::{Playfield, Position},
    tiles::{Tetromino, Tile},
};
use rand::Rng;

pub fn new_random_tetromino() -> Tetromino {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..7) {
        0 => Tetromino::i(),
        1 => Tetromino::j(),
        2 => Tetromino::l(),
        3 => Tetromino::o(),
        4 => Tetromino::s(),
        5 => Tetromino::t(),
        _ => Tetromino::z(),
    }
}

#[derive(Debug)]
pub struct FallingTetromino {
    tetromino: Tetromino,
    position: Position,
}

#[derive(Debug)]
pub struct Game {
    playfield: Playfield,
    active_tile: FallingTetromino,
}

impl Game {
    pub fn new() -> Self {
        let position = (Playfield::COLS / 2 - 2, 1).into();
        // let tetromino = new_random_tetromino();
        let tetromino = Tetromino::j();
        let mut playfield = Playfield::default();
        playfield.add(&tetromino, &position);
        Self {
            playfield,
            active_tile: FallingTetromino {
                tetromino,
                position,
            },
        }
    }
    pub fn fall(&mut self) -> &mut Self {
        let (x, y) = (self.active_tile.position.0, self.active_tile.position.1);
        let tiles: &[Tile] = self.active_tile.tetromino.grid().into();
        tiles.chunks(4).enumerate().for_each(|(i, chunk)| {
            let new_x = (y + i + 1) * Playfield::COLS + x;
            self.playfield.field[new_x..new_x + 4].copy_from_slice(chunk);
        });

        self.active_tile.position.1 += 1;
        // let new_x = (y) * Playfield::COLS + x;
        // self.playfield.field[new_x..new_x + 4].copy_from_slice(&[Tile::Empty; 4]);

        self
    }
    // pub fn fall(&mut self) -> &mut Self {
    //     let (x, y) = (self.active_tile.position.0, self.active_tile.position.1);
    //     let tiles: &mut [Tile] = self.active_tile.tetromino.grid_mut().0.borrow_mut(); // Utter
    //                                                                                    // horse
    //                                                                                    // shit
    //     tiles.chunks_mut(4).enumerate().for_each(|(i, chunk)| {
    //         let new_x = (y + i + 1) * Playfield::COLS + x;
    //         self.playfield.field.iter_mut().skip(new_x).take(4).enumerate().for_each(|(j, f)| {
    //             if f != &Tile::Filled {
    //                 *f = chunk[j]
    //             }
    //         });
    //         chunk.iter().for_each(|c| if c == &Tile::Filled {
    //             let clear_x = y * Playfield::COLS + x;
    //             self.playfield.field[clear_x..clear_x+ 4].copy_from_slice(&[Tile::Empty; 4]);
    //         });
    //     });
    //     self.active_tile.position.1 += 1;
    //
    //     self
    // }
    pub fn check_colision(&mut self) -> bool {
        let tiles: &[Tile] = self.active_tile.tetromino.grid().into();
        let position = &self.active_tile.position;
        let (x, y) = (position.0, position.1);
        // falling colision
        tiles.chunks(4).rev().take(1).any(|chunk| {
            chunk
                .iter()
                .enumerate()
                .filter(|(_, &c)| c == Tile::Filled)
                .any(|(pos, _)| {
                    let position_below = (y + 4) * Playfield::COLS + (x + pos);
                    y + 4 >= Playfield::ROWS || self.playfield.field[position_below] == Tile::Filled
                })
        })
    }
    pub fn new_tetromino(&mut self, tetromino: Tetromino) {
        let position = Position::default();
        self.playfield.add(&tetromino, &position);
        self.active_tile.tetromino = tetromino;
        self.active_tile.position = position;
        // let tiles: &[Tile] = tetromino.grid().into();
        // let (x, y) = (position.0, position.1);
        // tiles.chunks(4).enumerate().rev().for_each(|(i, chunk)| {
        //     let new_x = (y + i) * Playfield::ROWS + x;
        //     self.playfield.field[new_x + 4..new_x].copy_from_slice(chunk);
        // });
    }
    pub fn playfield(&self) -> &Playfield {
        &self.playfield
    }
}
