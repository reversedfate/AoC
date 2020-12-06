use std::fs;

extern crate common;

const DEBUG: bool = false;

fn main(){
    debug_log("Day 1!");

    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines().collect::<Vec<&str>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 
    
    debug_log("Searching pair of numbers...");
    let pair = find_pair(&input_lines, 2020);
    let (first, second) = pair;
    debug_log(&format!("Found: {}, {}", first, second)); 

    println!("Result (part1): {}", first * second); 
    
    debug_log("Searching pair of numbers...");
    let triple = find_triple(input_lines, 2020);
    let (first, second, third) = triple;
    debug_log(&format!("Found: {}, {}, {}", first, second, third)); 

    println!("Result (part2): {}", first * second * third);  
}

fn find_pair(input_lines: &Vec<&str>, expected_sum: u32) -> (u32, u32) {
    let mut number_lines: Vec<u32> = input_lines.iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    for i in 0..number_lines.len() {
        for j in i..number_lines.len() {
            // debug_log(&format!("Checking {}=>{}, {}=>{}, sum={}", i, number_lines[i], j, number_lines[j], number_lines[i] + number_lines[j])); 
            if number_lines[i] + number_lines[j] == expected_sum
            && i!=j {
                return (number_lines[i], number_lines[j]);
            }
        }
    }
    panic!("No two numbers match expected value!");
}

fn find_triple(input_lines: Vec<&str>, expected_sum: u32) -> (u32, u32, u32) {
    let mut number_lines: Vec<u32> = input_lines.iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    for i in 0..number_lines.len() {
        for j in i..number_lines.len() {
            for k in j..number_lines.len() {
                if number_lines[i] + number_lines[j] + number_lines[k] == expected_sum
                && (i!=k && i!=j && k!=j) {
                    return (number_lines[i], number_lines[j], number_lines[k]);
                }
            }
        }
    }    
    panic!("No three numbers match expected value!");
}

pub fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}