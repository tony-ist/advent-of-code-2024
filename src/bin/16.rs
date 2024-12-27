use std::collections::HashSet;
use advent_of_code::{Board, Bounded, Coord, Searchable, Vector};

advent_of_code::solution!(16);

const STEP_WEIGHT: u32 = 1;
const TURN_WEIGHT: u32 = 1000;

struct WeightedGraph {
    nodes_amount: usize,
    /// Node with id i has always index i in this vector
    nodes: Vec<Option<Node>>,
    edges: Vec<Vec<Option<u32>>>,
}

impl WeightedGraph {
    pub fn new(max_capacity: usize) -> WeightedGraph {
        let mut nodes = Vec::with_capacity(max_capacity);
        let mut row: Vec<Option<u32>> = Vec::with_capacity(max_capacity);

        for _ in 0..max_capacity {
            nodes.push(None);
            row.push(None);
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

    pub fn find_node_by_tag(&self, tag: char) -> Option<usize> {
        for i in 0..self.nodes_amount {
            if let Some(maybe_node) = self.nodes.get(i) {
                if let Some(node) = maybe_node {
                    if tag == node.tag {
                        return Some(node.id);
                    }
                }
            }
        }
        return None;
    }

    pub fn find_node_id_by_coord(&self, coord: &Coord) -> Option<usize> {
        for i in 0..self.nodes_amount {
            if let Some(maybe_node) = self.nodes.get(i) {
                if let Some(node) = maybe_node {
                    if coord.eq(&node.coord) {
                       return Some(node.id);
                    }
                }
            }
        }
        return None;
    }

    pub fn shortest_path_cost(&self, start_node_id: usize, end_node_id: usize) -> Distance {
        let mut distances: Vec<Distance> = Vec::with_capacity(self.nodes_amount);
        let mut node_set: HashSet<usize> = HashSet::new();
        for i in 0..self.nodes_amount {
            distances.push(Distance::new(None));
            node_set.insert(i);
        }
        distances[start_node_id] = Distance::new(Some(0));

        while !node_set.is_empty() {
            let node_id = pop_node_with_min_dist(&distances, &mut node_set);
            
            let neighbours = self.neighbours(node_id);
            
            for neighbour in neighbours {
                if !node_set.contains(&neighbour) {
                    continue;
                }
                
                let alt = distances[node_id].add(self.distance(node_id, neighbour));
                
                if alt.less_than(&distances[neighbour]) {
                    distances[neighbour] = alt;
                }
            }
        }

        return distances[end_node_id].clone();
    }
    
    pub fn neighbours(&self, node_id: usize) -> Vec<usize> {
        todo!()
    }
    
    pub fn distance(&self, from_node_id: usize, to_node_id: usize) -> &Distance {
        return self.edges[from_node_id][to_node_id];
    }

    pub fn add_node(&mut self, tag: char, coord: Coord, direction: Vector) -> usize {
        let new_node_id = self.nodes_amount;
        self.nodes[new_node_id] = Some(Node::new(new_node_id, tag, coord, direction));
        self.nodes_amount += 1;
        return new_node_id;
    }

    pub fn add_edge_by_id(&mut self, from_node_id: usize, to_node_id: usize, weight: u32) {
        self.edges[from_node_id][to_node_id] = Some(weight);
    }

    pub fn add_edge_by_coord(&mut self, coord1: &Coord, coord2: &Coord, weight: u32) {
        let node1_id = self.find_node_id_by_coord(coord1).unwrap();
        let node2_id = self.find_node_id_by_coord(coord2).unwrap();
        self.add_edge_by_id(node1_id, node2_id, weight);
    }

    pub fn add_nodes_for_each_passable_cell(&mut self, board: &Board) {
        for i in 0..board.height() {
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
}

fn pop_node_with_min_dist(distances: &Vec<Distance>, node_set: &mut HashSet<usize>) -> usize {
    todo!()
}

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
}

fn convert_board_to_graph(board: &Board) -> WeightedGraph {
    let nodes_amount = board.width() * board.height() * 4;
    let mut graph = WeightedGraph::new(nodes_amount);

    graph.add_nodes_for_each_passable_cell(board);
    graph.add_edges_between_cells(board);
    
    return graph;
}

fn score(board: &Board) -> u32 {
    let graph = convert_board_to_graph(board);
    let start = graph.find_node_by_tag('S').unwrap();
    let end = graph.find_node_by_tag('E').unwrap();
    let result = graph.shortest_path_cost(start, end).value();
    return result;
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
