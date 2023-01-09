use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()

    // let file = File::open(filename).expect("no such file");
    // let buf = BufReader::new(file);
    // buf.lines()
    //     .map(|l| l.expect("could not parse line"))
    //     .collect()
}

fn main() {
    // let lines = lines_from_file("./input.txt").expect("could not load input file");
    // let sum = advent_of_code::get_summed_priority(lines);
    // println!("sum: {}", sum);

    let lines = lines_from_file("./input.txt").expect("could not load input file");
    let sum = advent_of_code::get_summed_badges(lines);
    println!("day 2 sum: {}", sum);
}
