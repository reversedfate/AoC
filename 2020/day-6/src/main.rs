extern crate common;
mod group;

const DEBUG: bool = true;

fn main() {
    println!("Day N!");
    
    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines().collect::<Vec<&str>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 
    
    let (answer_1, answer_2) = process_lines(&input_lines);
    println!("Answer (part 1) = {}", answer_1);
    println!("Answer (part 2) = {}", answer_2);
}

pub fn process_lines(input_lines: &Vec<&str>) -> (usize, usize) {
    let mut total = 0;
    let mut total_2 = 0;
    
    let mut grp = group::group::Group::new();
    
    for line_number in 0..input_lines.len() {
        let line = input_lines[line_number];
        if line == "".to_string() {
            total += grp.get_unique_letter_count();
            total_2 += grp.get_letter_count_in_all();
            grp = group::group::Group::new();
        } else {
            grp.add_line(line);
        }   
    }
    total += grp.get_unique_letter_count();
    total_2 += grp.get_letter_count_in_all();

    return (total, total_2);
}

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn test_1() {
        let input_lines: Vec<&str> = vec!
            ["a",
            "a s d", 
            "d s e"];
        assert_eq!((4, 0), process_lines(&input_lines));
    }

    #[test]
    fn test_2() {
        let input_lines: Vec<&str> = vec!
            [
                "abc",
                "",
                "a",
                "b",
                "c",
                "",
                "ab",
                "ac",
                "",
                "a",
                "a",
                "a",
                "a",
                "",
                "b"
            ];
        assert_eq!((11, 6), process_lines(&input_lines));
    }
 
}