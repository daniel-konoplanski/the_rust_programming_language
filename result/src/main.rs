use std::{
    error,
    fs::File,
    io::ErrorKind,
    io::{Read, Write},
};

fn example1()
{
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

fn example2()
{
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn main()
{
    // example1();
    example2();
}
