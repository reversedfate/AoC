use petgraph::graph::Graph;
use petgraph::Outgoing;
use petgraph::Directed;
use petgraph::algo::has_path_connecting;
use petgraph::dot::Dot;
use regex::Regex;

const DEBUG: bool = true;

fn main() {
    println!("Day 7!");
    
    debug_log("Getting input...");
    let input = common::read_input("./input.txt");
    let input_lines = input.lines().collect::<Vec<&str>>();
    debug_log(&format!("Number of lines: {}", input_lines.len())); 

    // construct bag graph
    let mut bag_graph : Graph<String, u32, Directed> = Graph::new();
    input_to_graph(&input_lines, &mut bag_graph);
    
    // find out what nodes can lead to "shiny gold"
    let target_node_name = "shiny gold";
    let count = target_bag_can_be_contained_in(target_node_name, &bag_graph);
    let count2 = how_many_bags_bag_contains(target_node_name, &bag_graph);

    println!("Answer (part 1) = {}", count);
    println!("Answer (part 2) = {}", count2);
}

// part 1
fn target_bag_can_be_contained_in(target_node_name: &str, bag_graph: &Graph<String, u32, Directed>) -> u32 {
    let mut count : u32 = 0;
    
    if !bag_graph.node_indices().any(|j| bag_graph[j]==target_node_name) {
        debug_log(&format!("Graph does not contain target node '{}'!", target_node_name));
        return count;
    }

    let golden_key = bag_graph.node_indices().find(|i| bag_graph[*i] == target_node_name).unwrap();
    debug_log(&format!("Checking if target node '{}' can be contained in other bags...", target_node_name));
    
    for index in bag_graph.node_indices() {
        if index != golden_key {
            if petgraph::algo::has_path_connecting(
                &bag_graph,
                index,
                golden_key,
                None
            ) {
                count += 1;
            }
        }
    }

    return count;
}

// part 2
fn how_many_bags_bag_contains(target_node_name: &str, bag_graph: &Graph<String, u32, Directed>) -> u32 {
    if !bag_graph.node_indices().any(|j| bag_graph[j]==target_node_name) {
        debug_log(&format!("Graph does not contain target node '{}'!", target_node_name));
        return 0;
    }
    let target_node_index = bag_graph.node_indices().find(|i| bag_graph[*i] == target_node_name).unwrap();
    return how_many_bags_bag_contains_recursive(target_node_index, &bag_graph);
}


fn how_many_bags_bag_contains_recursive(target_node_index: petgraph::prelude::NodeIndex, bag_graph: &Graph<String, u32, Directed>) -> u32 {
    let mut count = 0;

    let mut edges = bag_graph.neighbors_directed(target_node_index, Outgoing).detach();
    while let Some(edge) = edges.next_edge(&bag_graph) {
        let (from_node, to_node) = bag_graph.edge_endpoints(edge).unwrap();
        count += bag_graph[edge] * (1 + how_many_bags_bag_contains_recursive(to_node, &bag_graph));
    }

    return count;
}
 
fn input_to_graph<'a>(input_lines: &'a Vec<&'a str>, bag_graph: &'a mut Graph<String, u32>) {
    let regex_main_bag: Regex = Regex::new(r"^(.+) bags contain").unwrap();
    let regex_child_bags: Regex = Regex::new(r"(?P<number>[0-9]+) (?P<name>.+?) bag").unwrap();
    
    for i in 0..input_lines.len() {
        debug_log(&format!("Checking input line '{}'...", input_lines[i]));
        
        // parse the line into useful parts
        let mut child_bags: Vec<(u32, &str)> = Vec::new();
        
        // parse line and get main bag name
        let captures = regex_main_bag.captures(&input_lines[i]);       
        let main_bag = captures.unwrap().get(1).unwrap().as_str();
        debug_log(&format!("    Main bag is '{}' and ...", main_bag));
        if !bag_graph.node_indices().any(|i| bag_graph[i]==main_bag) {
            bag_graph.add_node(main_bag.to_owned());   
        }
        let main_bag_index = bag_graph.node_indices().find(|i| bag_graph[*i] == main_bag).unwrap();
    
        // parse line and get contained bag names and number
        for caps in regex_child_bags.captures_iter(&input_lines[i]) {
            debug_log(&format!("    It requires '{}' - '{}'", caps["number"].parse::<u32>().unwrap(), &caps["name"]));
        
            if !bag_graph.node_indices().any(|j| bag_graph[j]==&caps["name"]) {
                bag_graph.add_node(caps["name"].to_owned());   
            }
            let child_bag_index = bag_graph.node_indices().find(|i| bag_graph[*i] == &caps["name"]).unwrap();
    
            bag_graph.add_edge(
                main_bag_index, 
                child_bag_index, 
                caps["number"].parse::<u32>().unwrap());  
        }
    }
}

fn debug_log(str: &str) {
    if DEBUG {println!("{}", str)};
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn empty_graph() {
        let input = "";
        let input_lines = input.lines().collect::<Vec<&str>>();
        let mut bag_graph : Graph<String, u32, Directed> = Graph::new();
        input_to_graph(&input_lines, &mut bag_graph);
        
        let target_node_name = "shiny gold";
        let count_in_types = target_bag_can_be_contained_in(target_node_name, &bag_graph);
        let count_must_contain = how_many_bags_bag_contains(target_node_name, &bag_graph);

        assert_eq!(bag_graph.node_count(), 0);
        assert_eq!(bag_graph.edge_count(), 0);        
        assert_eq!(count_in_types, 0);
        assert_eq!(count_must_contain, 0);
    }

    #[test]
    fn single_bag_one_child() {
        let input = "light orange bags contain 1 shiny gold bags.";

        let input_lines = input.lines().collect::<Vec<&str>>();
        let mut bag_graph : Graph<String, u32, Directed> = Graph::new();
        input_to_graph(&input_lines, &mut bag_graph);
        
        let target_node_name = "shiny gold";
        let count_in_types = target_bag_can_be_contained_in(target_node_name, &bag_graph);
        let count_must_contain = how_many_bags_bag_contains(target_node_name, &bag_graph);

        assert_eq!(bag_graph.node_count(), 2);
        assert_eq!(bag_graph.edge_count(), 1);        
        assert_eq!(count_in_types, 1);
        assert_eq!(count_must_contain, 0);        
    }
    
    #[test]
    fn more_complex_case() {
        let mut input = "A bags contain 1 shiny gold bag.".to_owned();
        input.push_str("\nB bags contain 111 A bags and 222 shiny gold bags.");
        input.push_str("\nC bags contain 1111 shiny gold bags.");
        input.push_str("\nshiny gold bags contain 2 D bags.");

        let input_lines = input.lines().collect::<Vec<&str>>();
        let mut bag_graph : Graph<String, u32, Directed> = Graph::new();
        input_to_graph(&input_lines, &mut bag_graph);
        
        let target_node_name = "shiny gold";
        let count_in_types = target_bag_can_be_contained_in(target_node_name, &bag_graph);
        let count_must_contain = how_many_bags_bag_contains(target_node_name, &bag_graph);

        assert_eq!(bag_graph.node_count(), 5);
        assert_eq!(bag_graph.edge_count(), 5);        
        assert_eq!(count_in_types, 3);
        assert_eq!(count_must_contain, 2);        
    }

    #[test]
    fn recursive_case() {
        let mut input = "shiny gold bags contain 2 A bags and 3 B bags.".to_owned();
        input.push_str("\nB bags contain 5 A bags and 2 C bags.");
        input.push_str("\nC bags contain 1111 D bags.");

        let input_lines = input.lines().collect::<Vec<&str>>();
        let mut bag_graph : Graph<String, u32, Directed> = Graph::new();
        input_to_graph(&input_lines, &mut bag_graph);
        
        let target_node_name = "shiny gold";
        let count_in_types = target_bag_can_be_contained_in(target_node_name, &bag_graph);
        let count_must_contain = how_many_bags_bag_contains(target_node_name, &bag_graph);

        assert_eq!(bag_graph.node_count(), 5);
        assert_eq!(bag_graph.edge_count(), 5);        
        assert_eq!(count_in_types, 0);
        assert_eq!(count_must_contain, 2+2*(0) + 3+3*(5+5*(0) + 2+2*(1111)));    
    }
}