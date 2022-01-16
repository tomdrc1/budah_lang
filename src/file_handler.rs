use std::fs;

pub fn read_file(file_path: String)
{
    let file_contents =  fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("{}", file_contents);
}