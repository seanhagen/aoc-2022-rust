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

    println!(
        "start of packet: {}",
        advent_of_code::find_packet_start(lines.first().unwrap().to_string())
    );

    println!(
        "start of message: {}",
        advent_of_code::find_message_start(lines.first().unwrap().to_string())
    );
}
