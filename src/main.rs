use core::fmt;

struct Grid([Tile; 16]);
#[derive(Copy, Clone)]
enum Tile {
    Filled,
    Empty,
}
enum Tetromino {
    I(Grid),
    J(Grid),
    L(Grid),
    O(Grid),
    S(Grid),
    T(Grid),
    Z(Grid),
}

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

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match &self {
            Tile::Filled => 'X',
            Tile::Empty => ' ',
        };
        write!(f, "{}", text)
    }
}

impl fmt::Display for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &c) in self.0.iter().enumerate() {
            write!(f, "{}", c)?;
            if (i + 1) % 4 == 0 && i < 15 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
impl Tetromino {
    pub fn i() -> Self {
        Tetromino::I(Grid::new([(0, 0), (1, 0), (2, 0), (3, 0)]))
    }
    pub fn j() -> Self {
        Tetromino::J(Grid::new([(3, 0), (3, 1), (2, 1), (1, 1)]))
    }
    pub fn l() -> Self {
        Tetromino::L(Grid::new([(1, 0), (2, 0), (3, 0), (3, 1)]))
    }
    pub fn o() -> Self {
        Tetromino::O(Grid::new([(2, 0), (3, 0), (2, 1), (3, 1)]))
    }
    pub fn s() -> Self {
        Tetromino::S(Grid::new([(3, 0), (2, 1), (3, 1), (2, 2)]))
    }
    pub fn t() -> Self {
        Tetromino::T(Grid::new([(3, 0), (2, 1), (3, 1), (3, 2)]))
    }
    pub fn z() -> Self {
        Tetromino::Z(Grid::new([(2, 0), (2, 1), (3, 1), (3, 2)]))
    }

    pub fn inspect(&self) -> &Grid {
        match self {
            Tetromino::I(grid)
            | Tetromino::J(grid)
            | Tetromino::L(grid)
            | Tetromino::O(grid)
            | Tetromino::S(grid)
            | Tetromino::T(grid)
            | Tetromino::Z(grid) => grid,
        }
    }
    // pub fn rotate(&self) {todo!()}
}
fn main() {
    println!("{}\n ", Tetromino::j());
    println!("{}\n ", Tetromino::i());
    println!("{}\n ", Tetromino::l());
    println!("{}\n ", Tetromino::o());
    println!("{}\n ", Tetromino::s());
    println!("{}\n ", Tetromino::t());
    println!("{}\n ", Tetromino::z());
}
