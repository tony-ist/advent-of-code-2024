use std::collections::HashSet;
use itertools::Itertools;
use advent_of_code::{Board, Bounded, Coord, Searchable, Vector};

advent_of_code::solution!(16);

const STEP_WEIGHT: u32 = 1;
const TURN_WEIGHT: u32 = 1000;

struct WeightedGraph {
    nodes_amount: usize,
    /// Node with id i has always index i in this vector
    nodes: Vec<Option<Node>>,
    edges: Vec<Vec<Distance>>,
}

impl WeightedGraph {
    pub fn new(max_capacity: usize) -> WeightedGraph {
        let mut nodes = Vec::with_capacity(max_capacity);
        let mut row: Vec<Distance> = Vec::with_capacity(max_capacity);

        for _ in 0..max_capacity {
            nodes.push(None);
            row.push(Distance::INFINITY);
        }

        let mut edges = Vec::with_capacity(max_capacity);

        for _ in 0..max_capacity {
            edges.push(row.clone());
        }

        return WeightedGraph {
            nodes_amount: 0,
            nodes,
            edges,
        }
    }

    pub fn find_node_by_tag_and_direction(&self, tag: char, direction: Vector) -> Option<usize> {
        for i in 0..self.nodes_amount {
            if let Some(maybe_node) = self.nodes.get(i) {
                if let Some(node) = maybe_node {
                    if tag == node.tag && direction == node.direction {
                        return Some(node.id);
                    }
                }
            }
        }
        return None;
    }

    pub fn find_node_id_by_coord_and_direction(&self, coord: &Coord, direction: &Vector) -> Option<usize> {
        for i in 0..self.nodes_amount {
            if let Some(maybe_node) = self.nodes.get(i) {
                if let Some(node) = maybe_node {
                    if coord.eq(&node.coord) && direction == &node.direction {
                        return Some(node.id);
                    }
                }
            }
        }
        return None;
    }

    pub fn shortest_path(&self, start_node_id: usize, end_node_ids: Vec<usize>) -> FindPathResult {
        let mut distances: Vec<Distance> = Vec::with_capacity(self.nodes_amount);
        let mut parents: Vec<Option<usize>> = Vec::new();
        let mut node_set: HashSet<usize> = HashSet::new();
        for i in 0..self.nodes_amount {
            parents.push(None);
            distances.push(Distance::new(None));
            node_set.insert(i);
        }
        distances[start_node_id] = Distance::new(Some(0));
        let mut maybe_final_node_id = None;

        while !node_set.is_empty() {
            let node_id = pop_node_with_min_dist(&distances, &mut node_set);

            println!("Nodes left: {}", node_set.len());
            
            if end_node_ids.contains(&node_id) {
                maybe_final_node_id = Some(node_id);
                break;
            }
            
            let neighbour_ids = self.neighbours(node_id);

            for neighbour_id in neighbour_ids {
                if !node_set.contains(&neighbour_id) {
                    continue;
                }

                let alt = distances[node_id].add(self.distance(node_id, neighbour_id));

                if alt.less_than(&distances[neighbour_id]) {
                    distances[neighbour_id] = alt;
                    parents[neighbour_id] = Some(node_id);
                }
            }
        }
        
        let final_node_id = maybe_final_node_id.unwrap();
        let shortest_path = self.trace_path(&parents, final_node_id);
        
        let result = FindPathResult { 
            path: shortest_path,
            distance: distances[final_node_id].clone(),
        };

        return result;
    }

    pub fn neighbours(&self, node_id: usize) -> Vec<usize> {
        let mut result = Vec::new();
        for i in 0..self.nodes_amount {
            if self.edges[node_id][i].is_finite() {
                result.push(i);
            }
        }
        return result;
    }

    pub fn distance(&self, from_node_id: usize, to_node_id: usize) -> &Distance {
        return &self.edges[from_node_id][to_node_id];
    }

    pub fn add_node(&mut self, tag: char, coord: Coord, direction: Vector) -> usize {
        let new_node_id = self.nodes_amount;
        self.nodes[new_node_id] = Some(Node::new(new_node_id, tag, coord, direction));
        self.nodes_amount += 1;
        return new_node_id;
    }

    pub fn add_edge_by_id(&mut self, from_node_id: usize, to_node_id: usize, weight: u32) {
        self.edges[from_node_id][to_node_id] = Distance::new(Some(weight));
    }

    pub fn add_edge_by_coord(&mut self, coord1: &Coord, coord2: &Coord, weight: u32) {
        let vector = coord2.subtract(coord1);
        let node1_id = self.find_node_id_by_coord_and_direction(coord1, &vector).unwrap();
        let node2_id = self.find_node_id_by_coord_and_direction(coord2, &vector).unwrap();
        self.add_edge_by_id(node1_id, node2_id, weight);
    }

    pub fn add_nodes_for_each_passable_cell(&mut self, board: &Board) {
        for i in 0..board.height() {
            println!("add_nodes_for_each_passable_cell i: {}", i);
            for j in 0..board.width() {
                let coord = Coord::new(i as i32, j as i32);
                let tag = board.at(&coord).unwrap();
                if tag != '#' {
                    self.add_nodes_and_edges_for_coord(&coord, tag);
                }
            }
        }
    }

    pub fn add_edges_between_cells(&mut self, board: &Board) {
        for i in 0..board.height() {
            for j in 0..board.width() {
                let coord = Coord::new(i as i32, j as i32);
                let tag = board.at(&coord).unwrap();
                if tag == '#' {
                    continue;
                }

                [Vector::NORTH, Vector::WEST, Vector::EAST, Vector::SOUTH].iter().for_each(
                    |direction| {
                        let offset_coord = coord.add(direction);
                        if let Some(tag) = board.at(&offset_coord) {
                            if tag != '#' {
                                self.add_edge_by_coord(&coord, &offset_coord, STEP_WEIGHT);
                            }
                        }
                    }
                );
            }
        }
    }
    pub fn add_nodes_and_edges_for_coord(&mut self, coord: &Coord, tag: char) {
        let node_north_id = self.add_node(tag, coord.clone(), Vector::NORTH.clone());
        let node_east_id = self.add_node(tag, coord.clone(), Vector::EAST.clone());
        let node_south_id = self.add_node(tag, coord.clone(), Vector::SOUTH.clone());
        let node_west_id = self.add_node(tag, coord.clone(), Vector::WEST.clone());

        self.add_edge_by_id(node_north_id, node_east_id, TURN_WEIGHT);
        self.add_edge_by_id(node_east_id, node_north_id, TURN_WEIGHT);

        self.add_edge_by_id(node_north_id, node_west_id, TURN_WEIGHT);
        self.add_edge_by_id(node_west_id, node_north_id, TURN_WEIGHT);

        self.add_edge_by_id(node_south_id, node_east_id, TURN_WEIGHT);
        self.add_edge_by_id(node_east_id, node_south_id, TURN_WEIGHT);

        self.add_edge_by_id(node_south_id, node_west_id, TURN_WEIGHT);
        self.add_edge_by_id(node_west_id, node_south_id, TURN_WEIGHT);
    }

    pub fn trace_path(&self, parents: &Vec<Option<usize>>, end_node_id: usize) -> Vec<Coord> {
        let mut maybe_previous = parents[end_node_id];
        let mut result = Vec::new();
        result.push(self.nodes[end_node_id].clone().unwrap().coord);
        
        while let Some(previous) = maybe_previous {
            maybe_previous = parents[previous];
            result.push(self.nodes[previous].clone().unwrap().coord);
        }
        
        return result;
    }
    
    pub fn find_nodes_by_tag(&self, tag: char) -> Vec<usize> {
        let mut result = Vec::new();
        
        for maybe_node in &self.nodes {
            if let Some(node) = maybe_node {
                if node.tag == tag {
                    result.push(node.id);
                }
            }
        }
        
        return result;
    }
}

fn pop_node_with_min_dist(distances: &Vec<Distance>, node_set: &mut HashSet<usize>) -> usize {
    let mut distance = &Distance::INFINITY;
    let mut maybe_result = None;
    for &node in node_set.iter() {
        if distances[node].less_than(distance) {
            maybe_result = Some(node);
            distance = &distances[node];
        }
    }
    let result = maybe_result.unwrap();
    node_set.remove(&result);
    return result;
}

struct FindPathResult {
    pub path: Vec<Coord>,
    pub distance: Distance,
}

#[derive(Clone, Debug)]
struct Node {
    pub id: usize,
    pub tag: char,
    pub coord: Coord,
    pub direction: Vector,
}

impl Node {
    pub fn new(id: usize, tag: char, coord: Coord, direction: Vector) -> Node {
        return Node { id, tag, coord, direction };
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Distance(Option<u32>);

impl Distance {
    pub const INFINITY: Distance = Distance(None);
    
    pub fn new(option: Option<u32>) -> Distance {
        Distance { 0: option }
    }
    
    pub fn add(&self, distance: &Distance) -> Distance {
        return match self.0 {
            Some(lhs) => match distance.0 {
                Some(rhs) => Distance::new(Some(lhs + rhs)),
                None => Distance::new(None),
            },
            None => Distance::new(None),
        };
    }
    
    pub fn less_than(&self, distance: &Distance) -> bool {
        return match self.0 {
            Some(lhs) => match distance.0 {
                Some(rhs) => lhs < rhs,
                None => true,
            },
            None => false,
        }
    }
    
    pub fn value(&self) -> u32 {
        return self.0.unwrap();
    }
    
    pub fn is_finite(&self) -> bool {
        return self.0.is_some();
    }
}

fn convert_board_to_graph(board: &Board) -> WeightedGraph {
    let nodes_amount = board.width() * board.height() * 4;
    println!("nodes_amount: {}", nodes_amount);
    let mut graph = WeightedGraph::new(nodes_amount);
    println!("WeightedGraph::new(nodes_amount) complete");
    
    graph.add_nodes_for_each_passable_cell(board);
    println!("add_nodes_for_each_passable_cell complete");
    graph.add_edges_between_cells(board);
    println!("add_edges_between_cells complete");
    
    return graph;
}

fn score(board: &Board) -> u32 {
    println!("Start scoring...");
    let graph = convert_board_to_graph(board);
    println!("convert_board_to_graph complete");
    let start_direction = Vector::EAST;
    let start_node_id = graph.find_node_by_tag_and_direction('S', start_direction).unwrap();
    let end_node_ids = graph.find_nodes_by_tag('E');
    let path_result = graph.shortest_path(start_node_id, end_node_ids);
    println!("shortest_path complete");
    let mut board = board.clone();
    trace_path_on_board(path_result.path, &mut board);
    println!("trace_path_on_board complete");
    println!("{}", &board);
    return path_result.distance.value();
}

fn trace_path_on_board(path: Vec<Coord>, board: &mut Board) {
    for coord in path.iter() {
        let current = board.at(coord);
        if current == Some('.') {
            board.mutate(coord, 'o');
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::from(input);
    return Some(score(&board));
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_east() {
        let input = [
          "####",  
          "#SE#",  
          "####",  
        ].join("\n");
        let board = Board::from(&input);
        let actual = score(&board);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_score_north() {
        let input = [
            "###",
            "#E#",
            "#S#",
            "###",
        ].join("\n");
        let board = Board::from(&input);
        let actual = score(&board);
        assert_eq!(actual, 1001);
    }

    #[test]
    fn test_distance_inf_less_than_inf() {
        let dist1 = Distance::new(None);
        let dist2 = Distance::new(None);
        assert_eq!(dist1.less_than(&dist2), false);
    }

    #[test]
    fn test_distance_some_less_than_inf() {
        let dist1 = Distance::new(Some(42));
        let dist2 = Distance::new(None);
        assert_eq!(dist1.less_than(&dist2), true);
    }

    #[test]
    fn test_distance_inf_less_than_some() {
        let dist1 = Distance::new(None);
        let dist2 = Distance::new(Some(42));
        assert_eq!(dist1.less_than(&dist2), false);
    }

    #[test]
    fn test_distance_some_less_than_some_true() {
        let dist1 = Distance::new(Some(41));
        let dist2 = Distance::new(Some(42));
        assert_eq!(dist1.less_than(&dist2), true);
    }

    #[test]
    fn test_distance_some_less_than_some_false() {
        let dist1 = Distance::new(Some(43));
        let dist2 = Distance::new(Some(42));
        assert_eq!(dist1.less_than(&dist2), false);
    }

    #[test]
    fn test_distance_add() {
        let dist1 = Distance::new(Some(43));
        let dist2 = Distance::new(Some(42));
        assert_eq!(dist1.add(&dist2), Distance::new(Some(85)));
    }

    #[test]
    fn test_distance_inf() {
        let dist1 = Distance::new(Some(43));
        let dist2 = Distance::new(None);
        assert_eq!(dist1.add(&dist2), Distance::new(None));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
