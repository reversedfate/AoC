extern crate common;
mod sled;

const DEBUG: bool = true;
const TREE_SYMBOL: char = '#';


fn main() {
    println!("Day N!");
    
    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines().collect::<Vec<&str>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 

    let trees_hit = count_trees((3,1), &input_lines);
    println!("Answer (part 1) = {}", trees_hit);

    let multiplied_trees = try_different_slopes(&input_lines);
    println!("Answer (part 2) = {}", multiplied_trees);
}

fn try_different_slopes(input_lines: &Vec<&str>) -> u64 {
    let mut multiplication = count_trees((1, 1), input_lines) as u64;
    multiplication *= count_trees((3, 1), input_lines) as u64;
    multiplication *= count_trees((5, 1), input_lines) as u64;
    multiplication *= count_trees((7, 1), input_lines) as u64;
    multiplication *= count_trees((1, 2), input_lines) as u64;
    return multiplication;
}

fn count_trees(slope: (i32, i32), input_lines: &Vec<&str>) -> i32 {
    let mut sled = sled::Sled::new();
    sled.set_position((0, 0));
    sled.set_slope(slope);
    
    let (x_slope, y_slope) = slope;
    debug_log(&format!("Counting trees on slope ({}, {})...", x_slope, y_slope));
    
    let height = get_height(input_lines);    
    let mut trees = 0;
    for t in 0..height {
        let position = sled.get_position_at_time(t as i32);
        if position_has_tree(position, &input_lines) {
            trees += 1;
        }
    } 
    
    debug_log(&format!("Trees = {}", trees));
    return trees;
}

fn get_height(input_lines: &Vec<&str>) -> usize {
    return input_lines.len();
}

fn get_width(input_lines: &Vec<&str>) -> usize {
    return input_lines[0].len();
}


fn normalize_position(position: (i32, i32), input_lines: &Vec<&str>) -> (usize, usize) {
    // returns position as input positions
    let (x, y) = position;
    let width = get_width(input_lines);
    return (x as usize % width, y as usize);
}

fn position_has_tree(position: (i32, i32), input_lines: &Vec<&str>) -> bool {
    let (x, y) = position;
    debug_log(&format!("Checking position ({}, {})", x, y));
    let (x, y) = normalize_position(position, input_lines);
    debug_log(&format!("When normalized it is equal to ({}, {})", x, y));

    let height = get_height(input_lines);
    if y > height {
        // panic!("looking for a line outside of input");
        return false;
    }

    let symbol = input_lines[y].chars().collect::<Vec<char>>()[x];

    if symbol == TREE_SYMBOL {
        return true;
    }
    return false;
}

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}