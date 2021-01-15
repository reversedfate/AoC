extern crate common;

const DEBUG: bool = true;

fn main() {
    println!("Day 8!");
    
    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines().map(String::from).collect::<Vec<String>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 
    
    let (answer1, _) = run_instructions(&input_lines);
    let (answer2, finished2) = get_accumulator_after_fixed(&input_lines);

    println!("Answer (part 1) = {}", answer1);
        
    if finished2 {
        println!("Answer (part 2) = {}", answer2);
    } else {
        println!("Part 2 has no answer!");
    }
}

fn run_instructions(input_lines : &Vec<String>) -> (i32, bool) {
    if input_lines.len() == 0 {
        return (0, false);
    }
    
    let mut accumulator = 0;
    let mut next_command_index = 0;
    let mut visit_counts : Vec<u32> = vec![0; input_lines.len()];
    // debug_log(&format!("visit_counts looks like this {:?}", visit_counts));
    let mut exit_condition = false;

    
    while !exit_condition {
        // debug_log(&format!("Visiting command \"{}\" on line {} (it has been visited {} times before)", &input_lines[next_command_index], next_command_index, visit_counts[next_command_index]));
        if next_command_index == input_lines.len() {
            return (accumulator, true);
        } 

        visit_counts[next_command_index] += 1;
        
        if visit_counts[next_command_index] < 2 {
            let (new_accumulator, new_next_command_index) = execute_line(&input_lines[next_command_index], accumulator, next_command_index as i32);
            accumulator = new_accumulator;
            next_command_index = new_next_command_index as usize;

            // debug_log(&format!("After execution accumulator = {}, next_command_index = {}", accumulator, next_command_index));
        } else {
            // debug_log(&format!("Stopping, because this line has been visited already!"));
            exit_condition = true;
        }
    }
    
    return (accumulator, false);
}


fn get_accumulator_after_fixed(input_lines : &Vec<String>) -> (i32, bool) {
    let valid_swap_count = count_instructions(input_lines, vec!["acc", "nop"]);
    debug_log(&format!("There are {} possible swaps in the original instructions", valid_swap_count));

    for i in 0..valid_swap_count as usize {
        let new_instructions: Vec<String> = switch_nth_op_with_op(&input_lines, i, ("jmp", "nop")); 

        debug_log(&format!("Trying to run instructions where {}. jmp/nop is replaced with nop/jmp...", i));
        let (accumulator, finished) = run_instructions(&new_instructions);
        debug_log(&format!("After running, accumulator={}, and finished = {}.", accumulator, finished));
        if finished {return (accumulator, true)};
    } 
    
    return (0, false);
}

fn count_instructions(input_lines : &Vec<String>, ops: Vec<&str>) -> u32 {
    let mut count = 0;
    
    for i in 0..input_lines.len() {
        let (line_op, _) = parse_line(&input_lines[i]);
        if ops.iter().any(|&i| i==line_op) {
            count += 1;
        }
    }
    
    return count;
}

fn switch_nth_op_with_op<'a>(input_lines : &'a Vec<String>, n: usize, ops: (&str, &str)) -> Vec<String> {
    //generate new set of instructions where nth instucion A is replaced with instruction B, or vice versa 
    let mut result: Vec<String> = input_lines.clone();

    let mut matches = 0;
    let (op1, op2) = ops;
    for i in 0..input_lines.len() {
        let (op, _) = parse_line(&input_lines[i]);
        
        if op == op1 || op == op2 { // match found
            if matches == n { // replace only the n-th match
                if op == op1 {
                    // result[i] = &result[i;
                    result[i] = result[i].replace(op1, op2);
                }
                if op == op2 {
                    result[i] = result[i].replace(op2, op1);
                }
            } 
            matches += 1;
        }
    }
    return result;
}

fn execute_line(line: &str, mut accumulator: i32, mut next_command_index: i32) -> (i32, i32) {
   let (op, number) = parse_line(line);
    match op {
       "acc" => {return execute_acc(number, accumulator, next_command_index);},
       "jmp" => {return execute_jmp(number, accumulator, next_command_index);},
       "nop" => {return execute_nop(number, accumulator, next_command_index);},
       _ => panic!(format!("No such op = {}", op)),
    } 
}

fn execute_acc( number: i32, mut accumulator: i32, mut next_command_index: i32 ) -> (i32, i32) {
    accumulator += number;
    next_command_index += 1;
    return (accumulator, next_command_index);
}

fn execute_jmp( number: i32, mut accumulator: i32, mut next_command_index: i32 ) -> (i32, i32) {
    next_command_index += number;
    return (accumulator, next_command_index);
}

fn execute_nop( number: i32, mut accumulator: i32, mut next_command_index: i32 ) -> (i32, i32) {
    next_command_index += 1;
    return (accumulator, next_command_index);
}

fn parse_line(line : &str) -> (&str, i32) {
    let parts : Vec<&str> = line.split(" ").collect();
    
    let op  = parts[0];    
    let number  = parts[1].parse::<i32>().unwrap();    

    return (op, number);
}

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn count_parts_in_strings(){
        assert_eq!(count_instructions(&vec!["jmp +321".to_owned()], vec!["jmp", "nop"]), 1);
        assert_eq!(count_instructions(&vec!["jmp +321".to_owned(),
                                            "acc +0".to_owned(),
                                            "nop -321".to_owned()],
                                        vec!["jmp", "nop"]),
                                    2);
    }

    #[test]
    fn parse_all_kinds_of_lines() {
        assert_eq!(parse_line("add +42"), ("add", 42));
        assert_eq!(parse_line("jmp +236"), ("jmp", 236));
        assert_eq!(parse_line("acc +43"), ("acc", 43));
        assert_eq!(parse_line("acc +28"), ("acc", 28));
        assert_eq!(parse_line("jmp +149"), ("jmp", 149));
        assert_eq!(parse_line("acc +28"), ("acc", 28));
        assert_eq!(parse_line("acc +13"), ("acc", 13));
        assert_eq!(parse_line("acc +36"), ("acc", 36));
        assert_eq!(parse_line("acc +42"), ("acc", 42));
        assert_eq!(parse_line("jmp +439"), ("jmp", 439));
        assert_eq!(parse_line("acc -14"), ("acc", -14));
        assert_eq!(parse_line("jmp +29"), ("jmp", 29));
        assert_eq!(parse_line("jmp +154"), ("jmp", 154));
        assert_eq!(parse_line("acc +16"), ("acc", 16));
        assert_eq!(parse_line("acc -13"), ("acc", -13));
        assert_eq!(parse_line("acc -16"), ("acc", -16));
        
        assert_ne!(parse_line("add +42"), ("add", -42));
        assert_ne!(parse_line("jmp +236"), ("jmpp", 236));
        assert_ne!(parse_line("acc +43"), ("acc", 403));
        assert_ne!(parse_line("acc +28"), ("acc", 0));
        assert_ne!(parse_line("jmp +149"), ("", 149));
    }

    #[test]
    fn empty_input() {
        let input = "";
        let input_lines = input.lines().map(String::from).collect::<Vec<String>>();

        let (answer1, _) = run_instructions(&input_lines);
        let (answer2, finished2) = get_accumulator_after_fixed(&input_lines);

        assert_eq!(answer1, 0);
        assert_eq!(answer2, 0);
        assert_eq!(finished2, false);
    }

    #[test]
    fn single_instruction__no_loop() {
        let input = "acc +9001".to_owned();
        let input_lines = input.lines().map(String::from).collect::<Vec<String>>();

        let (answer1, _) = run_instructions(&input_lines);
        let (answer2, finished2) = get_accumulator_after_fixed(&input_lines);

        assert_eq!(answer1, 9001);
        assert_eq!(answer2, 9001);
        assert_eq!(finished2, true);
    }

    #[test]
    fn simple_loop() {
        let mut input = "jmp +0".to_owned();
        input.push_str("\nacc +11");
        let input_lines = input.lines().map(String::from).collect::<Vec<String>>();

        let (answer1, _) = run_instructions(&input_lines);
        let (answer2, finished2) = get_accumulator_after_fixed(&input_lines);

        assert_eq!(answer1, 0);
        assert_eq!(answer2, 11);
        assert_eq!(finished2, true);
    }

    #[test]
    fn unresolvable_loop() {
        let mut input = "jmp +0".to_owned();
        input.push_str("\nacc +11");
        input.push_str("\njmp +0");
        input.push_str("\nacc +11");
        let input_lines = input.lines().map(String::from).collect::<Vec<String>>();

        let (answer1, _) = run_instructions(&input_lines);
        let (answer2, finished2) = get_accumulator_after_fixed(&input_lines);

        assert_eq!(answer1, 0);
        assert_eq!(answer2, 0);
        assert_eq!(finished2, false);
    }

    #[test]
    fn complex_loop() {
        let mut input = "nop +4".to_owned();
        input.push_str("\nacc +11");
        input.push_str("\njmp +3");
        input.push_str("\nacc +11");
        input.push_str("\nacc +11");
        input.push_str("\nacc +11");
        input.push_str("\njmp -5");
        input.push_str("\nacc +11");
        let input_lines = input.lines().map(String::from).collect::<Vec<String>>();

        let (answer1, _) = run_instructions(&input_lines);
        let (answer2, finished2) = get_accumulator_after_fixed(&input_lines);

        assert_eq!(answer1, 22); //0>1(+11)>2>5(+11)>6>1
        assert_eq!(answer2, 33); //replaces 3rd, 0>1(+11)>2>5>6>7(+11)>end
        assert_eq!(finished2, true);
    }
}