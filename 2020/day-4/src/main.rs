extern crate common;
mod passport;

const DEBUG: bool = true;

fn main() {
    println!("Day 4!");
    
    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines().collect::<Vec<&str>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 
    


    let (valid_passports, super_valid_passports) = count_valid_passports(&input_lines);
    println!("Answer (part 1) = {}", valid_passports);
    println!("Answer (part 2) = {}", super_valid_passports);
}

fn count_valid_passports(input_lines: &Vec<&str>) -> (u32, u32) {
    let mut valid_count = 0;
    let mut valid_count_2 = 0; 
    let mut passport: passport::Passport = passport::Passport::new();

    debug_log(&format!("Initial empty passport = {}", passport.to_string()));
    
    for line_number in 0..input_lines.len() {
        let line = input_lines[line_number];
        if line == "".to_string() {
            debug_log(&format!("Line is empty, checking if passport is valid, passport = {}.", passport.to_string()));
            if passport.is_valid() {
                debug_log(&format!("Valid!"));
                valid_count += 1;
            } else {
                debug_log(&format!("Invalid!"));
            }
            // part 2
            if passport.is_valid_2() {
                debug_log(&format!("Valid (part 2)!"));
                valid_count_2 += 1;
            } else {
                debug_log(&format!("Invalid (part 2)!"));
            }
            passport = passport::Passport::new();
        } else {
            debug_log(&format!("Adding string [{}] to passport.", line));
            debug_log(&format!("Before addition passport = {}", passport.to_string()));
            passport.set_data(line); // parsing logic happens in the structure method
            debug_log(&format!("After addition passport = {}", passport.to_string()));
        }        
    }

    debug_log(&format!("End of file checking last passport, passport = {}.", passport.to_string()));
    if passport.is_valid() {
        debug_log(&format!("Valid!"));
        valid_count += 1;
    } else {
        debug_log(&format!("Invalid!"));
    }
    // part 2
    if passport.is_valid_2() {
        debug_log(&format!("Valid (part 2)!"));
        valid_count_2 += 1;
    } else {
        debug_log(&format!("Invalid (part 2)!"));
    }

    return (valid_count, valid_count_2);
}

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}