use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub type LinesIter = std::io::Lines<std::io::BufReader<std::fs::File>>;

pub fn read_as_lines<T: ToString>(path: T) -> LinesIter {
    let file = File::open(PathBuf::from(path.to_string()))
        .expect("Cannot open file");

    BufReader::new(file).lines()
}