use std::process::exit;

use minesveiper::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Please specify exactly three (3) arguments: the number of columns (width), the number of rows (height) and the difficulty (1-100)");
        exit(1);
    }

    let width: usize = args[1].parse().expect("Invalid number");
    let height: usize = args[2].parse().expect("Invalid number");
    let diff: usize = args[3].parse().expect("Invalid number");

    let mut grid = Grid::new(height, width);
    grid.bombify(diff);
    grid.reveal_rnd();

    println!("{}", grid);
}
