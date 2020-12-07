extern crate common;

const DEBUG: bool = true;

fn main() {
    println!("Day N!");
    
    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines().collect::<Vec<&str>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 
    
    println!("Answer (part 1) = {}", 0);
    println!("Answer (part 2) = {}", 0);
}

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}