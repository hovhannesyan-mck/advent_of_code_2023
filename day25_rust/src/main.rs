use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::{fs, collections::HashMap};
use petgraph::graph::UnGraph;
use petgraph::dot::{Dot, Config};
use petgraph::unionfind::UnionFind;
use petgraph::visit::Dfs;
//use petgraph::algo::connected_components;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GraphNode {
    name: String,
    connections: Vec<String>,
}

impl GraphNode {
    fn new(name: String) -> Self {
        Self {
            name,
            connections: Vec::new(),
        }
    }
}

fn parse_data(
    contents: &String,
) -> HashMap<String, GraphNode> {
    let mut graph: HashMap<String, GraphNode> = HashMap::new();

    contents.lines().for_each(|line| {
        let split = line.split(": ").collect::<Vec<&str>>();

        let name = split[0].to_string();

        if !graph.contains_key(&name) {
            graph.insert(name.clone(), GraphNode::new(name.clone()));
        }

        let connections = split[1].split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

        connections.iter().for_each(|connection| {
            if !graph.contains_key(connection) {
                graph.insert(connection.clone(), GraphNode::new(connection.clone()));
            }

            graph.get_mut(&name).unwrap().connections.push(connection.clone());
            graph.get_mut(connection).unwrap().connections.push(name.clone());
        });
    });

    graph
}

fn part_1(
    contents: &String,
) -> i64 {
    let graph_map = parse_data(contents);
    
    let mut index_map = HashMap::new();
    let mut graph = UnGraph::<String, ()>::new_undirected();

    graph_map.keys().for_each(|key| {
        index_map.insert(key.clone(), graph.add_node(key.clone()));
    });

    graph_map.iter().for_each(|(key, node)| {
        node.connections.iter().for_each(|connection| {
            let first_node = index_map.get(key).unwrap();
            let second_node = index_map.get(connection).unwrap();
            graph.add_edge(*first_node, *second_node, ());
        });
    });

    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    
    let mut file = File::create("graph.dot").unwrap();
    write!(file, "{}", format!("{:?}", dot)).unwrap();

    graph.remove_edge(graph.find_edge(
        *index_map.get("rpd").unwrap(), 
        *index_map.get("bnv").unwrap())
        .unwrap());

    graph.remove_edge(graph.find_edge(
        *index_map.get("vfh").unwrap(), 
        *index_map.get("bdj").unwrap())
        .unwrap());

    graph.remove_edge(graph.find_edge(
        *index_map.get("ttv").unwrap(), 
        *index_map.get("ztc").unwrap())
        .unwrap());

    graph.remove_edge(graph.find_edge(
        *index_map.get("bnv").unwrap(), 
        *index_map.get("rpd").unwrap())
        .unwrap());

    graph.remove_edge(graph.find_edge(
        *index_map.get("bdj").unwrap(), 
        *index_map.get("vfh").unwrap())
        .unwrap());

    graph.remove_edge(graph.find_edge(
        *index_map.get("ztc").unwrap(), 
        *index_map.get("ttv").unwrap())
        .unwrap());
    

    let mut visited = HashSet::new();
    let mut count1 = 0;
    let mut count2 = 0;
    let mut component = 1;
    
    for node in graph.node_indices() {
        if !visited.contains(&node) {
            let mut dfs = Dfs::new(&graph, node);
            while let Some(nx) = dfs.next(&graph) {
                visited.insert(nx);
                if component == 1 {
                    count1 += 1;
                } else {
                    count2 += 1;
                }
            }
            component += 1;
        }
    }

    count1 * count2
}

fn part_2(
    contents: &String,
) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 54);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 7);
    }
}
