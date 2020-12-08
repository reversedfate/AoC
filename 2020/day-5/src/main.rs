extern crate common;
use std::collections::HashMap;

const DEBUG: bool = false;

fn main() {
    println!("Day N!");
    
    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines().collect::<Vec<&str>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 
    
    let largest_index = find_largest_index(&input_lines);
    println!("Answer (part 1) = {}", largest_index);
    
    let my_seat_index = find_my_seat_index(&input_lines);
    println!("Answer (part 2) = {}", my_seat_index);
}

fn find_my_seat_index(input_lines: &Vec<&str>) -> u32 {
    let mut taken_positions = HashMap::new();
    let mut min_index = 0;
    let mut max_index = 0;

    for i in 0..input_lines.len(){
        let position = instructions_to_position(input_lines[i]);
        let index = position_to_index(position);
        if i == 0 {
            min_index = index;
            max_index = index;
        } else {
            if index > max_index {max_index = index};
            if index < min_index {min_index = index};
        } 

        taken_positions.insert(index, position);
    }

    let mut found_seat_index = 0;
    debug_log(&format!("Indexes vary from {} to {}.", min_index, max_index));
    for i in min_index..max_index {
        if taken_positions.contains_key(&i) {
            let (x,y) = taken_positions[&i];
            debug_log(&format!("Index {} is filled at position ({}, {}).", i, x, y));
        } else {
            debug_log(&format!("Index {} is missing!", i));
        }
        if taken_positions.contains_key(&(i-1)) 
        && taken_positions.contains_key(&(i+1))
        && !taken_positions.contains_key(&i) {
            debug_log(&format!("Potential seat id = {}.", i));
            found_seat_index = i;
        } 
    }

    return found_seat_index;
}

fn instructions_to_position(instructions: &str) -> (u32, u32) {
    let binary_instructions = instructions
    .replace("F", "0")
    .replace("B", "1")
    .replace("L", "0")
    .replace("R", "1");
    
    let row_binary_string = &binary_instructions[..7];
    let column_binary_string = &binary_instructions[7..10];
    
    
    let x: u32 = u32::from_str_radix(row_binary_string, 2).unwrap();
    let y: u32 = u32::from_str_radix(column_binary_string, 2).unwrap();
    
    debug_log(&format!("    Original instructions \"{}\" get translated into \"{}\", and then split into 2 parts [\"{}\",\"{}\"], which translate into [{}, {}]", 
                        instructions,
                        binary_instructions,
                        row_binary_string,
                        column_binary_string,
                        x,
                        y
                    ));
   
                    
    let position = (x, y);
    return position;   
}

fn position_to_index(position: (u32, u32)) -> u32 {
    let (x, y) = position;
    return x * 8 + y;
}

fn find_largest_index(input_lines: &Vec<&str>) -> u32 {
    let mut largest_index = 0;

    for i in 0..input_lines.len(){
        let position = instructions_to_position(input_lines[i]);
        let index = position_to_index(position);
        let (x, y) = position;
        // debug_log(&format!("Checking instructions {}, which corresponds to position [{},{}] and index {}", input_lines[i], x, y, index));
        if index > largest_index {
            largest_index = index;
        } 
    }

    return largest_index;
}

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}