use core::fmt;
use std::borrow::BorrowMut;


#[derive(Clone, Copy)]
pub struct Grid(pub [Tile; 16]);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Filled,
    Empty,
}
pub struct Tetromino(Grid);

impl Grid {
    pub fn new(positions: [(usize, usize); 4]) -> Self {
        positions
            .iter()
            .for_each(|&(x, y)| assert!(x < 4 && y < 4, "position {},{} is out of bounds", x, y));

        positions.iter().fold(Grid::default(), |mut grid, &(x, y)| {
            grid.0[x * 4 + y] = Tile::Filled;
            grid
        })
    }
}

impl From<Grid> for [Tile;16] {
    fn from(value: Grid) -> Self {
        value.0
    }
}
impl <'a> From<&'a Grid> for &'a [Tile] {
    fn from(value: &'a Grid) -> Self {
        value.0.as_ref()
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self([Tile::Empty; 16])
    }
}

impl Grid {
    pub fn transpose(&mut self) -> &mut Self {
        // 0 1 2 3    4 5 6 7    8 9 10 11   12 13 14 15
        // 0 4 8 12   1 5 9 13   2 6 10 14   3  7  11 15
        //
        // deleting the copies, gives us:
        //
        // 0 1 2 3    5 6 7    10 11   15
        // 0 4 8 12   5 9 13   10 14   15
        // let transposed_idx: Vec<usize> = (0..16).map(|i| (i % 4) * 4 + (i / 4)).collect();

        let grid = &mut self.0;
        grid.swap(1, 4);
        grid.swap(2, 8);
        grid.swap(3, 12);
        grid.swap(6, 9);
        grid.swap(7, 13);
        grid.swap(11, 14);
        self
    }
    pub fn reverse(&mut self) -> &mut Self {
        let grid = &mut self.0;
        grid.chunks_mut(4).for_each(|c| {
            c.swap(0, 3);
            c.swap(1, 2)
        });
        self
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match &self {
            Tile::Filled => 'X',
            Tile::Empty => 'o',
        };
        write!(f, "{}", text)
    }
}

impl fmt::Debug for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.grid())
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &c) in self.0.iter().enumerate() {
            write!(f, "{:?}", c)?;
            if (i + 1) % 4 == 0 && i < 15 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
#[allow(dead_code)]
impl Tetromino {
    pub fn i() -> Self {
        Tetromino(Grid::new([(0, 0), (1, 0), (2, 0), (3, 0)]))
    }
    pub fn j() -> Self {
        Tetromino(Grid::new([(3, 0), (3, 1), (2, 1), (1, 1)]))
    }
    pub fn l() -> Self {
        Tetromino(Grid::new([(1, 0), (2, 0), (3, 0), (3, 1)]))
    }
    pub fn o() -> Self {
        Tetromino(Grid::new([(2, 0), (3, 0), (2, 1), (3, 1)]))
    }
    pub fn s() -> Self {
        Tetromino(Grid::new([(3, 0), (2, 1), (3, 1), (2, 2)]))
    }
    pub fn t() -> Self {
        Tetromino(Grid::new([(3, 0), (2, 1), (3, 1), (3, 2)]))
    }
    pub fn z() -> Self {
        Tetromino(Grid::new([(2, 0), (2, 1), (3, 1), (3, 2)]))
    }

    pub fn grid_mut(&mut self) -> &mut Grid {
        self.0.borrow_mut()
    }
    pub fn grid(&self) -> &Grid {
        &self.0
    }
    pub fn rotate_clockwise(&mut self) -> &mut Self {
        self.grid_mut().transpose().reverse();
        self
    }
    pub fn rotate_anticlockwise(&mut self) -> &mut Self {
        self.grid_mut().transpose();
        self
    }
}
