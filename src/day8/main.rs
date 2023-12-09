use std::collections::HashMap;
use std::hash;
use std::fmt;
use std::env;
use std::fs;
use std::hash::Hasher;


#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    id: [u8; 3],
}

#[derive(Debug, Clone)]
struct Network {
    next_left: HashMap<Node, Node>,
    next_right: HashMap<Node, Node>
}

// -------------------------------------------------------------------------- //

impl Node {
    fn new(node_id: &str) -> Node {
        let id = node_id.trim();
        assert_eq!(id.len(), 3);
        let bytes: Vec<u8> = id.chars().map(|c| c as u8).collect();
        Node {
            id: bytes.try_into().unwrap(),
        }
    }

    fn is_end(&self) -> bool {
        self.id.iter().all(|c| *c == b'Z')
    }

    fn last_char(&self) -> char {
        self.id[2] as char
    }
}

impl hash::Hash for Node {
    fn hash<H: hash::Hasher>(&self, state: &mut H) where H: Hasher {
        self.id.hash(state)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from_iter(self.id.iter().map(|c| *c as char));
        write!(f, "Node {{ \"{}\" }}", s)
    }
}

// -------------------------------------------------------------------------- //

fn gcd(a: usize, b: usize) -> usize {
    let mut x = a;
    let mut y = b;
    while y > 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn part1(instructions: &str, network: &Network) -> usize {
    let mut steps = 0;
    let mut current_node = Node::new("AAA");

    'outer: loop {
        for lr in instructions.chars() {
            let next = match lr {
                'L' => network.next_left.get(&current_node).unwrap(),
                'R' => network.next_right.get(&current_node).unwrap(),
                _ => unreachable!(),
            };

            steps += 1;
            current_node = *next;

            if current_node.is_end() {
                break 'outer;
            }
        }
    }
    steps
}

fn part2(instructions: &str, network: &Network) -> usize {
    let mut nodes: Vec<Node> = Vec::new();

    for n in network.next_left.keys() {
        if n.last_char() == 'A' {
            nodes.push(*n)
        }
    }

    let substeps: Vec<usize> = nodes.iter().map(|start| {
        let mut step = 0;
        let mut node = *start;
        let mut inst = instructions.chars().cycle();
        let mut end: Option<(Node, usize)> = None;

        loop {
            let lr = inst.next().unwrap();
            let next = match lr {
                'L' => network.next_left.get(&node).unwrap(),
                'R' => network.next_right.get(&node).unwrap(),
                _ => unreachable!(),
            };
            step += 1;
            node = *next;
            if node.last_char() == 'Z' {
                if end.is_none() {
                    end = Some((node, step));
                } else {
                    let (n, s) = end.unwrap();
                    assert_eq!(node, n);
                    step = s;
                    break;
                }
            }
        }
        step
    }).collect();

    let lcm = substeps.into_iter().reduce(lcm).unwrap();
    lcm
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let mut lines = input.lines();
    let instructions = lines.next().unwrap().trim();
    assert_eq!(lines.next(), Some(""));

    let mut left_map: HashMap<Node, Node> = HashMap::new();
    let mut right_map: HashMap<Node, Node> = HashMap::new();

    for line in lines {
        let mut outer = line.split(" = ");
        let node = Node::new(outer.next().unwrap());

        let left_right = outer.next().unwrap().trim_matches(|c| c == '(' || c == ')');
        let mut inner = left_right.split(", ");
        let left = Node::new(inner.next().unwrap());
        let right = Node::new(inner.next().unwrap());

        left_map.insert(node, left);
        right_map.insert(node, right);
    }

    let network = Network { 
        next_left: left_map,
        next_right: right_map,
    };

    let p1 = part1(&instructions, &network);
    println!("Part 1 result: {}", p1);
    
    let p2 = part2(&instructions, &network);
    println!("Part 2 result = {}", p2);
}
