use std::env;
mod libs;
fn main() {
    let arguments = env::args().collect::<Vec<String>>();
    let len = arguments.len() as i32;
    if (len < 2) || (arguments[1] == "list".to_string()) {
        libs::output_tasks();
    } else if arguments[1] == "add".to_string() {
        libs::add(arguments, len)
    } else if arguments[1] == "remove".to_string() {
        libs::remove(arguments)
    }
}
