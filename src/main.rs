use std::env;
use std::fs;
mod libs;
use home::home_dir;

fn main() {
    let version = "1.2";
    if let Some(home_path) = home_dir() {
        let mut home_path = home_path;
        home_path.push(".todolist");
        
        if !home_path.exists() {
            if let Err(e) = fs::create_dir_all(&home_path) {
                eprintln!("Failed to create directory: {}", e);
                return;
            }
        }

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
        }else if arguments[1] == "version" {
            println!("{}", version);
             
        } 
        else {
            eprintln!("Invalid command:\nValid Commands:\n - list\n - add\n - remove\n version- outputs version\n - help");
        }
    } else {
        eprintln!("Could not find the home directory");
    }
}
