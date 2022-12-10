use advent_of_code::RockPaperScissor;
use std::fs;

fn main() {
    let mut game = RockPaperScissor::new();

    let contents = fs::read_to_string("input.txt").expect("unable to read input file");

    for line in contents.lines() {
        game.add(line);
    }

    println!("score: {}", game.score());
}
