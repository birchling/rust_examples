use std::fs;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_read_result = fs::read_to_string("./file_tht_exists.txt");

    match file_read_result {
        Ok(text) => println!("{text}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found creating backup...");
                
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                });
                
                println!("Done!");
            },
            other_error => {
                panic!("I don't know how to deal with: {:?}", other_error);
            }
        }
    }
}
