use std::fs;

pub fn read_input(filename:&str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong with reading the file!");
    // debug_log(&format!("Contents:\n{}", contents));
    return contents;
}