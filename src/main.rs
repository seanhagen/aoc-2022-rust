use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let lines = lines_from_file("./input.txt").expect("could not load input file");
    let fully_contained = advent_of_code::fully_contained_count(lines);
    println!("fully contained count: {}", fully_contained);
}
