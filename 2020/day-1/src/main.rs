use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use io::Result;
use io::Lines;
use io::BufReader;

const DEBUG: bool = true;

fn main(){
    debug_log("Day 1!");

    debug_log("Getting input...");
    let input: vec![] = read_lines("./input.txt");
    input.iter();
    debug_log("Getting input..."); 

    let result: u32 = calculate(input);
    println!("Result: {}", result);    
}

fn calculate(_input: Result<Lines<BufReader<File>>>) -> u32{    

    return 2020;
}

// TODO 2 common
fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}

// TODO 2 common
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(filename: &str) -> vec![] {
    let mut array = vec![];
    let mut file = File::open(filename);
    let lines: Lines<BufReader<File>> = BufReader::new(file).lines();
    array = lines.collect();
    return array;
}