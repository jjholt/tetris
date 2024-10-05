mod tiles;
mod playfield;

use tiles::Tetromino;
fn main() {
    let mut test = Tetromino::j();
    println!("{}\n", test);
    test.rotate_clockwise().rotate_clockwise();
    println!("{}\n", test);
    // println!("{}\n ", Tetromino::j());
    // println!("{}\n ", Tetromino::i());
    // println!("{}\n ", Tetromino::l());
    // println!("{}\n ", Tetromino::o());
    // println!("{}\n ", Tetromino::s());
    // println!("{}\n ", Tetromino::t());
    // println!("{}\n ", Tetromino::z());
}
