use std::env;
mod get_tasks;
mod add_task;
mod remove_task;
use std::io;
fn help(){
  println!(
            "
The following are the valid commands:
      help - displays this message
      list - lists all the tasks
       add - adds a task
    remove - removes a task
     clear - completely clears the list
    "
        )
}

fn main() {
    let os_args: Vec<String> = env::args().collect();
    //if no args given, output the commands
    if os_args.len() == 1 {
        help();
    }

  else if os_args[1] == "list" {
    get_tasks::output_tasks();
  }
  else if os_args[1] == "add" {
    println!("What task do you want to add?");
    let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
  let input = input.trim();
  add_task::append(input.to_string());
    
  }
  else if os_args[1] == "remove"{
    remove_task::handle_remove();
  }
  else if os_args[1] == "clear"{
    let mut input= String::new();
    println!("Are you sure (Y/n)?");
    io::stdin().read_line(&mut input).expect("Line Read Fail");
    if input.trim() == "" || input.trim().to_lowercase() == "y"{
      println!("-- Clearing Todo List --")
    } else {
      println!("Not Cleared")
    }
    }else{
      help();
    
  }
  
}
