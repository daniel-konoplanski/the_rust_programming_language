use std::{error, fs::File, io::{Read, Write}};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let Ok(mut file) = greeting_file_result else {
        println!("File doesn't exist");
        return;
    };

    let mut buff = String::new();

    if let Err(error) = file.read_to_string(&mut buff) {
        println!("Failed to read file {error}");
        return;
    };

    let lines = buff.split_terminator("\n");

    let mut line_number = 1;
    for line in lines {
        println!("{line_number}: {line}");
        line_number += 1;
    }
}
