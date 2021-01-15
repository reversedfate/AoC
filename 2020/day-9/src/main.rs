extern crate common;

const DEBUG: bool = true;

fn main() {
    println!("Day 9!");
    
    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines()
                        .map(|a | a.to_string().parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 

    let (answer1, found1) = find_first_invalid_number(&input_lines, 25);

    if found1 
    { println!("Answer (part 1) = {}", answer1); }
    else 
    { println!("No answer for part 1!"); }

    println!("Answer (part 2) = {}", 0);
}

fn can_sum_number_from_preamble(number: i64, preamble: Vec<i64>) -> bool {
    //check if any two numbers sum to the required number in the preamble vector
    for i in 0..preamble.len() {
        for j in i..preamble.len() {
            if i != j {
                if preamble[i] + preamble[j] == number {
                    return true;
                }
            }
        }    
    }
    return false;
}

fn construct_preamble(index: usize, preamble_size: usize, input_lines: &Vec<i64>) -> Vec<i64> {
    let mut from_index = index - preamble_size; 
    let to_index = index;

    if from_index < 0 
        { from_index = 0; }
    
    if from_index == to_index
        { return vec![input_lines[to_index]]; }

    return input_lines[from_index .. to_index].to_vec();
}

fn find_first_invalid_number(input_lines: &Vec<i64>, preamble_size: usize) -> (i64, bool) {    
    
    for i in preamble_size..input_lines.len() {
        let preamble = construct_preamble(i, preamble_size, &input_lines);
        if !can_sum_number_from_preamble(input_lines[i], preamble) {
            return (input_lines[i], true);
        }
    }

    return (0, false); 
}

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn preamble_summability() {
        assert_eq!(can_sum_number_from_preamble(1, vec![0,1]), true);
        assert_eq!(can_sum_number_from_preamble(1, vec![0,2]), false);
        assert_eq!(can_sum_number_from_preamble(12, vec![6,7,5]), true);
        assert_eq!(can_sum_number_from_preamble(11, vec![6,7,5]), true);
        assert_eq!(can_sum_number_from_preamble(19, vec![1,2,3,4,5,6,7,8,9,10]), true);
        assert_eq!(can_sum_number_from_preamble(0, vec![1,2,3,4,5,6,7,8,9,10]), false);
    }

    #[test]
    fn preamble_construction() {
        //minimalistic case
        assert_eq!(
            construct_preamble(1, 1, &vec![0,1]),
            vec![0]
        );
        
        //simple cases
        assert_eq!(
            construct_preamble(7, 3, &(0..8).collect()),
            vec![4,5,6]
        );
        assert_eq!(
            construct_preamble(4, 2, &vec![0,1,2,3,4,5,6]),
            vec![2,3]
        );
        assert_eq!(
            construct_preamble(4, 4, &vec![0,1,2,3,4,5,6]),
            vec![0,1,2,3]
        );
        assert_eq!(
            construct_preamble(6, 6, &vec![0,1,2,3,4,5,6]),
            vec![0,1,2,3,4,5]
        );

        //realistic cases
        assert_eq!(
            construct_preamble(26, 25, &(0..100).collect()),
            (1..26).collect::<Vec<i64>>()
        );
        assert_eq!(
            construct_preamble(25, 25, &(0..100).collect()),
            (0..25).collect::<Vec<i64>>()
        );
        assert_eq!(
            construct_preamble(100, 25, &(0..100).collect()),
            (75..100).collect::<Vec<i64>>()
        );
    }

    #[test]
    fn first_invalid_number(){
        assert_eq!(
            find_first_invalid_number(&vec![0,1,2,3,4,5,999], 5),
            (999, true)
        );
        assert_eq!(
            find_first_invalid_number(&vec![0,1,2,3,4,5], 5),
            (0, false)
        );
        assert_eq!(
            find_first_invalid_number(&vec![0,1,2,4,4,8], 5),
            (0, false)
        );
        assert_eq!(
            find_first_invalid_number(&vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576], 5),
            (127, true)
        );
    }
}