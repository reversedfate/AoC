extern crate common;

const DEBUG: bool = true;

fn main() {
    println!("Day 2!");
    
    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines().collect::<Vec<&str>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 
    
    debug_log("Counting valid passwords...");
    let valid_count = count_correct_passwords(&input_lines);
    println!("Answer (part 1) = {}", valid_count);

    debug_log("Counting valid passwords again...");
    let valid_count = count_correct_passwords_2(input_lines);
    println!("Answer (part 2) = {}", valid_count);
}

fn count_correct_passwords(database_lines: &Vec<&str>) -> u32 {
    let mut count: u32 = 0;
    
    for i in 0..database_lines.len() {
        let database_line_parts = database_lines[i].split(": ").collect::<Vec<&str>>();
        let policy: &str = database_line_parts[0];
        let password: &str = database_line_parts[1];
        if is_password_valid(password, policy) {
            count+=1;
        }
    }

    return count;
}

fn count_correct_passwords_2(database_lines: Vec<&str>) -> u32 {
    let mut count: u32 = 0;
    
    for i in 0..database_lines.len() {
        let database_line_parts = database_lines[i].split(": ").collect::<Vec<&str>>();
        let policy: &str = database_line_parts[0];
        let password: &str = database_line_parts[1];
        if is_password_valid_2(password, policy) {
            count+=1;
        }
    }

    return count;
}

fn is_password_valid(password: &str, policy: &str) -> bool{
    let (ch, min, max) = parse_policy(policy);

    let mut ch_count = 0;
    for password_char in password.chars() {
        if password_char == ch {
            ch_count += 1;
        }
    } 
    let valid: bool = ch_count >= min && ch_count <= max;

    // debug_log(&format!("Checking if password {} works with policy [ch={},min={},max={}]. Character {} appears {} times, and thus it is {}", password, ch, min, max, ch, ch_count, if (valid) {"valid"} else {"invalid"}));

    return valid;
}

fn is_password_valid_2(password: &str, policy: &str) -> bool{
    let (ch, mut pos1, mut pos2) = parse_policy(policy);
    
    pos1 -= 1;
    pos2 -= 1;

    let mut ch_count = 0;
    let password_chars: Vec<char> = password.chars().collect();
    
    if (pos1 as usize) < password_chars.len() && password_chars[pos1 as usize] == ch { ch_count+=1 };
    if (pos2 as usize) < password_chars.len() && password_chars[pos2 as usize] == ch { ch_count+=1 };
    
    let valid: bool = ch_count == 1;
    
    return valid;
}

fn parse_policy(policy: &str) -> (char, u32, u32){
    let policy_parts = policy.split(" ").collect::<Vec<&str>>();
    let policy_number_parts = policy_parts[0].split("-").collect::<Vec<&str>>();

    let ch = policy_parts[1].chars().nth(0).unwrap();
    let min = policy_number_parts[0].parse::<u32>().unwrap();
    let max = policy_number_parts[1].parse::<u32>().unwrap();

    return (ch, min, max);
} 

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}
