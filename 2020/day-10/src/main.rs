extern crate common;
use std::collections::HashMap;

const DEBUG: bool = true;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn main() {
    println!("Day 10!");
    
    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines()
                            .map(|a | a.to_string().parse::<u32>().unwrap())
                            .collect::<Vec<u32>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 
    
    println!("Answer (part 1) = {}", get_part1_answer(&input_lines));
    println!("Answer (part 2) = {}", 0);
}

fn get_valid_adapter_order(input_lines: &Vec<u32>) -> Vec<u32> {
    let device_joltage = input_lines.iter().max().unwrap() + 3;
    let mut result: Vec<u32> = input_lines.clone();
    
    result.push(0);
    result.push(device_joltage);
    result.sort();
    
    return result;
}

fn count_element_differences_in(vector: &Vec<u32>) -> HashMap<i32, i32> {
    //returns a new vector that has counts as elements at index that matches the difference length
    let mut result: HashMap::<i32, i32> = HashMap::new();

    let differences = vector.iter()
                            .map(|&a| a as i32)
                            .zip(vector[1..].iter().map(|&a| a as i32))
                            .map(|(a, b)| b - a)
                            .collect::<Vec<i32>>();
                            // .group_by(|a| a);
    
    for difference in differences.iter() {
        if result.contains_key(difference) {
            *result.get_mut(difference).unwrap() += 1;
        } else {
            result.insert(*difference, 1);
        }
    }

    return result;
}

fn get_part1_answer(input_lines: &Vec<u32>) -> i32 {
    let differences: &HashMap<i32,i32> = &count_element_differences_in(&get_valid_adapter_order(&input_lines));

    if differences.contains_key(&1) && differences.contains_key(&3) {
        return differences.get(&1).unwrap() * differences.get(&3).unwrap();
    }
    
    return 0;
}

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn count_element_differences() {
        assert_eq!(
            count_element_differences_in(&vec![1,2,3,6]),
            map!{1 => 2, 3 => 1}
        );
        assert_eq!(
            count_element_differences_in(&(0..10000).collect()),
            map!{1 => 9999}
        );
    }

    #[test]
    fn valid_order_differences() {
        let input = &vec![3,2,1];

        let valid_order = get_valid_adapter_order(input); 
        assert_eq!(
            valid_order,
            vec![1,2,3,6]
        );

        let differences = count_element_differences_in(&valid_order);
        assert_eq!(
            differences,
            map!{1 => 2, 3 => 1}
        );

        let answer1 = get_part1_answer(&input);
        assert_eq!(answer1, 2);
    }
}
