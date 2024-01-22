use std::fs::File;
use std::io::{Read, Write};

pub fn append(data: String) {
    let mut read_file = File::open("tasks.txt").expect("Failed to open file for reading");

    let mut buffer = String::new();
    if let Err(e) = read_file.read_to_string(&mut buffer) {
        eprintln!("Error reading from file: {}", e);
    }

    let data = data.trim();
  let task = &data;

    let data = String::from(data);

    let data = format!("{}{}", buffer, data);

    let text_content = format!("{}{}", data, "\n");

    let mut new_file = File::create("tasks.txt").expect("Failed to create file");

    new_file
        .write_all(text_content.as_bytes())
        .expect("Failed to write to file. ");
  println!("Task: {} added",task);
}
