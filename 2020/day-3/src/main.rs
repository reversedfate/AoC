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

struct Sled {
    x_offset: i32,
    y_offset: i32,
    initial_position: (i32, i32)
}

impl Sled {
    fn position_at_time(t:i32) -> (i32,i32){
        let (mod x, mod y) = self.initial_position;
        return (x,y);
    }
}

fn count_trees(slope: (i32, i32)) -> u32 {
    let (x_offset, y_offset) = slope;
    let position: (i32, i32) = (0, 0);

    return 0;
}

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}