use std::fs;

struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct NodeWithIndex {
    name: String,
    left_index: usize,
    right_index: usize,
}

pub fn main() {
    let content = fs::read_to_string("./src/day8/input.txt").expect("Could not read file");

    let lines = content.lines().collect::<Vec<&str>>();
    let instructions = lines[0];
    let mut nodes: Vec<Node> = Vec::new();

    for line in &lines[2..] {
        let node_split = line.split('=').collect::<Vec<&str>>();
        if node_split.len() != 2 {
            panic!("Invalid line")
        }
        let node_name = node_split[0].trim();

        let node_moves_split = node_split[1].replace("(", "").replace(")", "");
        let node_moves_split = node_moves_split.split(',').collect::<Vec<&str>>();

        if node_moves_split.len() != 2 {
            panic!("Invalid line")
        }

        let left_move = node_moves_split[0].trim();
        let right_move = node_moves_split[1].trim();

        nodes.push(Node {
            name: node_name.to_string(),
            left: left_move.to_string(),
            right: right_move.to_string(),
        });
    }

    let nodes_with_index = to_node_with_index(&nodes);

    // task 1
    let winning_index = nodes_with_index
        .iter()
        .position(|n| n.name == "ZZZ")
        .expect("Node not found");
    let starting_index = nodes_with_index
        .iter()
        .position(|n| n.name == "AAA")
        .expect("Node not found");
    let moves_count = go_to_winner(
        starting_index,
        instructions,
        &nodes_with_index,
        &vec![winning_index],
    );
    println!("Task 1: {}", moves_count); // 22199

    // task 2
    let starting_positions = nodes_with_index
        .iter()
        .enumerate()
        .filter(|(_, n)| n.name.ends_with("A"))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let mut winning_positions = nodes_with_index
        .iter()
        .enumerate()
        .filter(|(_, n)| n.name.ends_with("Z"))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    winning_positions.sort();

    // every starting position reaches the winning point after n moves
    // then it loops to the same winning node with the same number of n moves
    let moves: Vec<u32> = starting_positions
        .iter()
        .map(|p| go_to_winner(*p, instructions, &nodes_with_index, &winning_positions))
        .collect();

    // find the LCM of all moves
    let mut curr_lcm: u64 = moves[0] as u64;
    for i in 1..moves.len() {
        curr_lcm = lcm(curr_lcm, moves[i].into());
    }
    println!("Task 2: {}", curr_lcm); // 13334102464297
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn go_to_winner(
    current_node_index: usize,
    instructions: &str,
    nodes_with_index: &Vec<NodeWithIndex>,
    winners_indexes: &Vec<usize>,
) -> u32 {
    let mut current_node_index = current_node_index;
    let mut moves_count: u32 = 0;
    'outer: loop {
        for ins in instructions.chars() {
            let current_node = &nodes_with_index[current_node_index];
            if ins == 'L' {
                current_node_index = current_node.left_index;
            } else {
                current_node_index = current_node.right_index;
            }
            moves_count += 1;
            if winners_indexes.contains(&current_node_index) {
                break 'outer;
            }
        }
    }
    moves_count
}

fn to_node_with_index(nodes: &Vec<Node>) -> Vec<NodeWithIndex> {
    let mut nodes_with_index: Vec<NodeWithIndex> = Vec::new();
    for node in nodes {
        let left_index = nodes
            .iter()
            .position(|n| n.name == node.left)
            .expect("Node not found");
        let right_index = nodes
            .iter()
            .position(|n| n.name == node.right)
            .expect("Node not found");

        nodes_with_index.push(NodeWithIndex {
            name: node.name.clone(),
            left_index,
            right_index,
        });
    }
    nodes_with_index
}
