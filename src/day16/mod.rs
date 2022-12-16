use std::{path::Path, collections::{HashMap, HashSet, BinaryHeap}};

use crate::utils::{self, parse_i64};

type Node = String;

struct TimestepValues {
    values: HashMap<Node, i64>,
}

struct TimeBoard {
    // the 
    nodes_at_times: Vec<Node>
}

#[derive(Clone, Debug)]
struct Graph {
    valve_rates: HashMap<Node, i64>,
    connected_nodes: HashMap<Node, Vec<(Node, i64)>>,
}

impl<T: Iterator<Item = String>> From<T> for Graph {
    fn from(lines: T) -> Self {
        let mut valve_rates: HashMap<Node, i64>= HashMap::new();
        let mut connected_nodes: HashMap<Node, Vec<(Node, i64)>>= HashMap::new();
        for line in lines {
            let (node, remaining) = line
                .split_once("Valve ").unwrap().1
                .split_once(" has flow rate=").unwrap();
            let node = node.to_owned();
            let (flow_rate_str, remaining) = remaining 
                .split_once("; tunnel").unwrap();
            let flow_rate = parse_i64(flow_rate_str).unwrap();
            let connected_valves: Vec<(Node, i64)> = remaining
                .split_once("valve").unwrap().1
                .split_once(" ").unwrap().1
                .split(", ").map(|s| s.to_owned())
                .map(|node| (node, 1))
                .collect();
            valve_rates.insert(node.clone(), flow_rate);
            connected_nodes.insert(node, connected_valves);
        };
        Graph { valve_rates, connected_nodes }
    }
}

impl Graph {

    fn new() -> Self {
        let valve_rates: HashMap<Node, i64>= HashMap::new();
        let connected_nodes: HashMap<Node, Vec<(Node, i64)>>= HashMap::new();
        Graph { valve_rates, connected_nodes }
    }

    /// Makes self into a dense graph without 0 weight nodes
    fn simplify(&self) -> Self {
        let mut graph = Graph::new();
        for (node, valve_rate) in self.valve_rates.iter() {
            if valve_rate != &0 {
                let connected_nodes = self.bfs(node.clone())
                    .into_iter()
                    .filter(|(node, _)| self.valve_rates.get(node).unwrap() != &0)
                    .collect();
                graph.connected_nodes.insert(node.clone(), connected_nodes);
                graph.valve_rates.insert(node.clone(), *valve_rate);
            }
        };
        return graph;
    }

    fn bfs(&self, node: Node) -> HashMap<Node, i64> {
        let mut dists = HashMap::new();
        let mut pq: BinaryHeap<(i64, Node)> = BinaryHeap::new();
        pq.push((0, node));
        while let Some((dist, node)) = pq.pop() {
            let dist = -dist;
            if !dists.contains_key(&node) {
                for (connected_node, new_dist) in self.connected_nodes.get(&node).unwrap().iter() {
                    let combined_dist = dist + *new_dist;
                    pq.push((-combined_dist, connected_node.to_owned()))
                }
                dists.insert(node, dist);
            }
        }
        return dists;
    }

    fn max_path_from(&self, node: Node, max_dist: i64) -> i64 {
        let graph = self.simplify();
        let initial_distances: Vec<(Node, i64)>= self.bfs(node)
            .into_iter()
            .filter(|(node, _)| graph.valve_rates.contains_key(node))
            .collect();
        println!("{:?}", graph);
        println!("{:?}", initial_distances);
        let mut max_score = 0;
        for (start_node, start_dist) in initial_distances {
            println!("Explore {}", start_node);
            max_score = max_score.max(graph.dfs_non_repeating(&start_node, max_dist - start_dist).0);
            println!("");
        }
        return max_score;
    }

    fn dfs_non_repeating(&self, start_node: &Node, max_dist: i64) -> (i64, Vec<Node>) {
        let mut used_nodes = HashSet::new();
        let mut node_stack = Vec::new();
        let output = self.dfs_non_repeating_helper(
            &start_node, 
            &mut used_nodes, 
            max_dist, 
            0,
            0
        );
        return (output, node_stack);
    }

    fn dfs_non_repeating_helper(&self, node: &Node, used_nodes: &mut HashSet<Node>, dist_remaining: i64, max_score: i64, depth: usize) -> i64 {
        if dist_remaining > 0 && !used_nodes.contains(node){
            used_nodes.insert(node.clone());
            //println!("{}Looking under {}: Used: {:?}", "   ".repeat(depth), node, used_nodes);
            let message = format!("Node {} not in {:?}", node, self.valve_rates);
            let max_score = max_score + (*self.valve_rates.get(node).expect(message.as_str()) * (dist_remaining - 1));
            let max_score = self.connected_nodes.get(node).unwrap()
                .iter()
                .map(|(next_node, dist)| {
                    if !used_nodes.contains(next_node) {
                        let max_score = self.dfs_non_repeating_helper(
                            next_node, 
                            used_nodes, 
                            dist_remaining - dist - 1, 
                            max_score,
                            depth + 1
                        );
                        //println!("{}opened {} at {} with score {}", "   ".repeat(depth+1), next_node, dist_remaining, max_score);
                        (max_score, next_node)
                    } else {
                        (max_score, next_node)
                    }
                })
                .max()
                .unwrap_or((max_score, node));
            used_nodes.remove(node);
            return max_score.0;
        } else {
            return max_score
        }
    }
}

type TimeLeft = usize;

fn build_value_graph(max_time: usize, graph: &Graph) -> HashMap<(Node, TimeLeft), i64> {
    let mut time_values: HashMap<(Node, TimeLeft), i64> = HashMap::new();
    for (node, valve_rate) in graph.valve_rates.iter() {
        // initial values are all zero, since you can't do anything in the last time step
        time_values.insert((node.clone(), 0), 0);
        // The best you can do at any given node in the last second is just to release the valve
        time_values.insert((node.clone(), 1), *valve_rate);
    }
    for time_left in 2..max_time+1 {
        for (node, connected_nodes) in graph.connected_nodes.iter() {
            // best case if we pull the valve
            let valve_rate = graph.valve_rates.get(node).expect("it's there");
            let best_value_if_pull = connected_nodes.iter()
                .map(|connected_node| time_values.get(&(connected_node.0.to_owned(), time_left - 2)).unwrap())
                .max()
                .unwrap_or(&0);
            let best_value_if_pull = best_value_if_pull + valve_rate * (time_left as i64 - 1);
            // best case if we don't pull the valve
            let best_value_if_no_pull = connected_nodes.iter()
                .map(|connected_node| time_values.get(&(connected_node.0.to_owned(), time_left - 1)).unwrap())
                .max()
                .unwrap_or(&0);
            let best_value = best_value_if_pull.max(*best_value_if_no_pull);
            time_values.insert((node.to_owned(), time_left), best_value);
        }
    }
    return time_values;
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let lines = utils::read_input(path);
    let graph = Graph::from(lines);
    //let value_graph = build_value_graph(30, &mut graph);
    return graph.max_path_from("AA".to_owned(), 30).to_string();
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}