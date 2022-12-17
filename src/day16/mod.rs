use std::{path::Path, collections::{HashMap, HashSet, BinaryHeap}};

use itertools::{Permutations, Itertools};

use crate::utils::{self, parse_i64};

type Node = String;

#[derive(Clone, Copy, Debug)]
struct AgentState<'a> {
    curr_node: &'a Node,
    dist_remaining: i64,
}


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
        println!("Graph size: {:?}", graph.valve_rates.len());
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
        let mut max_score = 0;
        for (start_node, start_dist) in initial_distances {
            max_score = max_score.max(graph.dfs_non_repeating(&start_node, max_dist - start_dist));
        }
        return max_score;
    }

    fn dfs_non_repeating(&self, start_node: &Node, max_dist: i64) -> i64 {
        let mut used_nodes = HashSet::new();
        let output = self.dfs_non_repeating_helper(
            &start_node, 
            &mut used_nodes, 
            max_dist, 
            0,
            0
        );
        return output;
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

    fn max_bi_path_from(&self, node: Node, max_dist: i64) -> i64 {
        let graph = self.simplify();
        println!("Simplified graph size: {}", graph.valve_rates.len());
        let mut initial_distances: Vec<(Node, i64)>= self.bfs(node)
            .into_iter()
            .map(|(node, dist)| {println!("{}, {}", node, dist); (node, dist)})
            .filter(|(node, _)| graph.valve_rates.contains_key(node))
            .collect();
        let mut max_score = 0;
        println!("Starting analysis");
        initial_distances.sort_by(|a, b| {
            let a = graph.valve_rates.get(&a.0).unwrap();
            let b = graph.valve_rates.get(&b.0).unwrap();
            return (-a).cmp(&-b);
        });
        println!("Initial distance:  {:?}", initial_distances);
        let mut count = 0;
        for (i, first_point) in initial_distances.iter().enumerate() {
            for (j, second_point) in initial_distances.iter().enumerate() {
                if j > i {
                    count += 1;
                    let start_nodes = (&first_point.0, &second_point.0);
                    let max_dists = (max_dist - first_point.1, max_dist - second_point.1);
                    if start_nodes.0 < start_nodes.1 {
                        let score_from_this_start = graph.bi_dfs_non_repeating(start_nodes, max_dists);
                        println!("Start {:?}, {} / {}: score: {}", start_nodes, count, 105, score_from_this_start);
                        max_score = max_score.max(score_from_this_start);
                    }
                }
            }
        }
        return max_score;
    }

    fn bi_dfs_non_repeating(&self, start_nodes: (&Node, &Node), max_dists: (i64, i64)) -> i64 {
        let mut used_nodes = HashSet::new();
        used_nodes.insert(start_nodes.0.clone());
        used_nodes.insert(start_nodes.1.clone());
        let your_score = *self.valve_rates.get(start_nodes.0).unwrap() * (max_dists.0 - 1);
        let elephant_score = *self.valve_rates.get(start_nodes.1).unwrap() * (max_dists.1 - 1);
        let output = self.bi_dfs_non_repeating_helper(
            AgentState { curr_node: start_nodes.0, dist_remaining: max_dists.0 - 1 },
            AgentState { curr_node: start_nodes.1, dist_remaining: max_dists.1 - 1},
            &mut used_nodes, 
            your_score + elephant_score,
            0,
        );
        return output;
    }

    fn bi_dfs_non_repeating_helper(&self, your_state: AgentState, elephant_state: AgentState, used_nodes: &mut HashSet<Node>, score: i64, depth: usize) -> i64 {
        // you and the elephant are the same
        let you_move = self.bi_dfs_non_repeating_helper_you_move(your_state, elephant_state, used_nodes, score, depth);
        let elephant_moves = self.bi_dfs_non_repeating_helper_you_move(elephant_state, your_state, used_nodes, score, depth);
        return you_move.max(elephant_moves);
    }

    fn bi_dfs_non_repeating_helper_you_move(&self, your_state: AgentState, elephant_state: AgentState, used_nodes: &mut HashSet<Node>, score: i64, depth: usize) -> i64 {
        self.connected_nodes.get(your_state.curr_node).unwrap()
            .iter()
            .map(|(next_node, dist_to_next)| {
                let next_dist = your_state.dist_remaining - dist_to_next - 1;
                if next_dist > 0 && !used_nodes.contains(next_node) {
                    used_nodes.insert(next_node.clone());
                    let next_node_vent_rate = self.valve_rates.get(next_node).unwrap();
                    let point_gain = next_dist * (*next_node_vent_rate);
                    let next_score = score + point_gain;
                    let next_state = AgentState { curr_node: next_node, dist_remaining: next_dist };
                    //println!("{}Considering {} -> {} and gaining {} point with remaining dist {}", "   ".repeat(depth+1), your_state.curr_node, next_node, point_gain, next_dist + 1);
                    //println!("{}dist_to_next: {}, next_state: {:?}", "   ".repeat(depth+1), dist_to_next, next_state);
                    let best_score = self.bi_dfs_non_repeating_helper(
                        next_state, 
                        elephant_state, 
                        used_nodes, 
                        next_score, 
                        depth + 1
                    );
                    used_nodes.remove(next_node);
                    return best_score;
                } else {
                    return score;
                }
            })
            .max()
            .unwrap_or(score)
    }

    /*
    fn bi_dfs_non_repeating_helper(&self, nodes: (&Node, &Node), used_nodes: &mut HashSet<Node>, dists_remaining: (i64, i64), max_score: i64, depth: usize) -> i64 {
        let max_if_you_move = if dists_remaining.0 <= 1 {
            max_score
        } else {
            let mut max_score = max_score;
            for (your_next_node, your_dist_to_next) in self.connected_nodes.get(nodes.0).unwrap() {
                let your_dist_remaining = dists_remaining.0 - your_dist_to_next - 1;
                if !used_nodes.contains(your_next_node) {
                    used_nodes.insert(your_next_node.clone());
                    let next_score = max_score + (*self.valve_rates.get(node).expect(message.as_str()) * (dist_remaining - 1));
                    let sub_max_score = self.bi_dfs_non_repeating_helper(
                        nodes, 
                        used_nodes, 
                        dists_remaining, 
                        depth + 1,
                    );
                    used_nodes.remove(your_next_node);
                }
            }
            max_score
        };

        if dist_remaining > 0 {
            used_nodes.insert(nodes.0.clone());
            used_nodes.insert(nodes.1.clone());
            // case 1: 
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
        */
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
    let lines = utils::read_input(path);
    let graph = Graph::from(lines);
    //let value_graph = build_value_graph(30, &mut graph);
    return graph.max_bi_path_from("AA".to_owned(), 26).to_string();
}