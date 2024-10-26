use std::fs;
use std::io;

fn main() {
    let result = read_username_from_file();

    match result {
        Ok(username) => println!("{username}"),
        Err(e) => panic!("{e:?}"),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
