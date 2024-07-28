use std::env;
mod libs;
use home::home_dir;

fn main() {
    if let Some(home_path) = home_dir() {
        if let Err(e) = env::set_current_dir(&home_path) {
            eprintln!("Failed to change the working directory: {}", e);
            return;
        }

        let arguments: Vec<String> = env::args().collect();
        let len = arguments.len();

        if len < 2 || arguments[1] == "list" {
            if let Err(e) = libs::output_tasks() {
                eprintln!("Error displaying tasks: {}", e);
            }
        } else if arguments[1] == "add" {
            if let Err(e) = libs::add(&arguments) {
                eprintln!("Error adding task: {}", e);
            }
        } else if arguments[1] == "remove" {
            if let Err(e) = libs::remove(&arguments) {
                eprintln!("Error removing task: {}", e);
            }
        } else {
            eprintln!("Invalid command");
        }
    } else {
        eprintln!("Could not find the home directory");
    }
}
