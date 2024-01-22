use std::fs::File;
use std::io::Write;
pub fn clear(){

  let mut file = File::create("tasks.txt").unwrap();

  file.write_all(b"").expect("Failed to clear file");

}