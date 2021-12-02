extern crate common;
use std::collections::HashMap;
use petgraph::graph::Graph;
use petgraph::Outgoing;
use petgraph::prelude::NodeIndex;
use petgraph::Directed;

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
                            .map(|a | a.to_string().parse::<u64>().unwrap())
                            .collect::<Vec<u64>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 
    
    println!("Answer (part 1) = {}", get_part1_answer(&input_lines));
    println!("Answer (part 2) = {}", get_part2_answer(&input_lines));
}

fn get_valid_adapter_order(input_lines: &Vec<u64>) -> Vec<u64> {
    let device_joltage = input_lines.iter().max().unwrap() + 3;
    let mut result: Vec<u64> = input_lines.clone();
    
    result.push(0);
    result.push(device_joltage);
    result.sort();
    
    return result;
}

fn count_element_differences_in(vector: &Vec<u64>) -> HashMap<i64, i64> {
    //returns a new vector that has counts as elements at index that matches the difference length
    let mut result: HashMap::<i64, i64> = HashMap::new();

    let differences = vector.iter()
                            .map(|&a| a as i64)
                            .zip(vector[1..].iter().map(|&a| a as i64))
                            .map(|(a, b)| b - a)
                            .collect::<Vec<i64>>();
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

fn get_part1_answer(input_lines: &Vec<u64>) -> i64 {
    let differences: &HashMap<i64,i64> = &count_element_differences_in(&get_valid_adapter_order(&input_lines));

    if differences.contains_key(&1) && differences.contains_key(&3) {
        return differences.get(&1).unwrap() * differences.get(&3).unwrap();
    }
    
    return 0;
}

fn get_part2_answer(input_lines: &Vec<u64>) -> u64 {   
    let full_order = get_valid_adapter_order(&input_lines); //this returns sorted array
    let mut graph : Graph<u64, u64, Directed> = Graph::new();
    input_to_graph(&full_order, &mut graph); // create graph that contains all connections between adapters
    
    let start_index = graph.node_indices().find(|x| graph[*x] == 0).unwrap();
    let end_index = graph.node_indices().find(|x| graph[*x] == *input_lines.iter().max().unwrap()).unwrap();
        
    // call recursive backtracking function to count all ways from outlet to device
    return count_valid_ways_to_outlet(&graph, &start_index, &end_index, 0);    
}

fn input_to_graph<'a>(input_lines: &'a Vec<u64>, graph: &'a mut Graph<u64, u64>) {
    // fill the graph with all adapters (including 0 and end) and has all possible connections in them
    // nodes - for each element in vector create node, store all node indexes temporarily
    // edges - check next elements in vector and for each one which has a diff of less than 4, create connection
    // debug_log(&format!("Max value = {}.", input_lines.iter().max().unwrap()));

    for i in 0..input_lines.len() {
        // debug_log(&format!("Processing {}.", input_lines[i]));
        if !graph.node_indices().any(|x| graph[x]==i as u64) { 
            // add node if it does not yet exist
            graph.add_node(input_lines[i].to_owned());    
        }
        let main_index = graph.node_indices().find(|x| graph[*x] == input_lines[i]).unwrap();
        
        let neighbour_iter = input_lines.into_iter()
                            .filter( | &x | *x as i64 - input_lines[i] as i64 >= 1 
                                         && *x as i64 - input_lines[i] as i64 <= 3);

        for n in neighbour_iter {
            if !graph.node_indices().any(|x| graph[x] == *n) { 
                // add child index, if it does not exist
                graph.add_node(n.to_owned());
            } 
            // debug_log(&format!("Adding {} as child to {}.", n, input_lines[i]));
            let child_index = graph.node_indices().find(|x| graph[*x] == *n).unwrap();
            
            // add edge between nodes
            graph.add_edge(
                main_index,
                child_index,
                1);
        }
    }
}

fn count_valid_ways_to_outlet(graph: &Graph<u64, u64>, start_index: &NodeIndex, target_index: &NodeIndex, depth: u64) -> u64 {
    let mut result = 0;
    // debug_log(&format!("{}: Counting all the ways from {} to {}...", depth, graph.node_weight(*start_index).unwrap(), graph.node_weight(*target_index).unwrap()));
    
    let mut edges = graph.neighbors_directed(*start_index, Outgoing).detach();
    while let Some(edge) = edges.next_edge(&graph) {
        let (from_node, to_node) = graph.edge_endpoints(edge).unwrap();
        // debug_log(&format!("  {}: It has an edge from {} to {}.", depth, graph.node_weight(from_node).unwrap(), graph.node_weight(to_node).unwrap()));   
    }
    
    
    // find all neighbours
    let mut edges = graph.neighbors_directed(*start_index, Outgoing).detach();
    while let Some(edge) = edges.next_edge(&graph) {
        let (from_node, to_node) = graph.edge_endpoints(edge).unwrap();
        
        // debug_log(&format!("  {}: Checking edge from {} to {}.", depth, graph.node_weight(from_node).unwrap(), graph.node_weight(to_node).unwrap()));
        if to_node.index() == target_index.index() {
            // if any neighbour is target, add one to result
            // debug_log(&format!("  {}: Connection!", depth));
            result += 1;
        } else {
            // if neighbour is not target, call this function with neighbour id for each neighbour
            // debug_log(&format!("  {}: Checking if {} can lead to {}", depth, graph.node_weight(to_node).unwrap(), graph.node_weight(*target_index).unwrap()));
            result += count_valid_ways_to_outlet(&graph, &to_node, &target_index, depth + 1);
        }
    }
    if depth == 40 {
        debug_log(&format!("Finished checking node {} with depth {}, it has {} connections to target!", 
            graph.node_weight(*start_index).unwrap(),
            depth,
            result
        ));
    }
    return result;
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
            vec![0,1,2,3,6]
        );

        let differences = count_element_differences_in(&valid_order);
        assert_eq!(
            differences,
            map!{1 => 3, 3 => 1}
        );

        let answer1 = get_part1_answer(&input);
        assert_eq!(answer1, 3);
    }
}
