mod game;
mod playfield;
mod tiles;

use game::{new_random_tetromino, Game};
fn main() {
    let mut game = Game::new();
    // println!("{:#?}", game);
    println!("Game Stats:\n{:?}", game.playfield());
    for _ in 0..12 {
        if game.check_colision() {
            game.new_tetromino(new_random_tetromino());
            println!("colision detected");
            println!("{:?}", game.playfield());
        } else {
            game.fall();
            println!("{:?}", game.playfield());
        }
    }
    // let mut playfield = Playfield::default();
    // playfield.add(&Tetromino::j(), (3,0).into());
    // playfield.add(&new_random_tetromino(), (3,0).into());
    // println!("{:?}", playfield);
    // println!("{:?}\n", j);
    // j.rotate_clockwise().rotate_clockwise();
    // println!("{:?}\n", j);
    // println!("{}\n ", Tetromino::j());
    // println!("{}\n ", Tetromino::i());
    // println!("{}\n ", Tetromino::l());
    // println!("{}\n ", Tetromino::o());
    // println!("{}\n ", Tetromino::s());
    // println!("{}\n ", Tetromino::t());
    // println!("{}\n ", Tetromino::z());
}
