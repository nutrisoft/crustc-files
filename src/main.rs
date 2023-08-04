use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use std::fs::OpenOptions;


use std::io::Write;

fn main() {
    let content = fs::read_to_string("world.txt").expect("Could not read file");

    println!("File content: {}", content);

    let content_write = "Humanity should be the goal of politics";

    fs::write("humanity.txt", content_write).expect("Could not write to file");
    // reading from a file line by line
    let path = Path::new("file.txt");
    let file = File::open(&path).expect("file not found");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read file");
        println!("{}", line);
    }
    // writing to a file fine by line
    let mut file_write = File::create("movie.txt").expect("Cannot create file");

    writeln!(file_write, "Games of throne").expect("Cannot write to file");
    writeln!(file_write, "Harry Potten").expect("Cannot write to file");

    // appending to a file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("number.txt").expect("Cannot write to file");

    for i in 1..6 {
        writeln!(file, "Appending line number {}", i).expect("Cannot write to file");
    }
}
