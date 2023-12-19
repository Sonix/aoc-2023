use std::collections::HashMap;
use aoc_2023::read_lines_from_file;


#[derive(Debug)]
enum Instruction {
    LEFT,
    RIGHT,
}

impl Instruction {
    fn new (input: char) -> Instruction {
        match input {
            'L' => Instruction::LEFT,
            'R' => Instruction::RIGHT,
            _ => panic!("Cant parse {}", input)
        }
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn new(input: &String) -> Node {
        Node{
        id: input[0..input.find('=').unwrap()].trim().to_string(),
        left: input[input.find('(').unwrap()+1..input.find(',').unwrap()].trim().to_string(),
        right: input[input.find(',').unwrap()+1..input.find(')').unwrap()].trim().to_string(),
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a,b) * b
}
fn main() {
    let lines = read_lines_from_file("day8.input");
    let instructions = lines[0].chars().map(Instruction::new).collect::<Vec<_>>();
    for instruction in &instructions {
        println!("{:?}", instruction);
    }

    let nodes = lines[2..].iter().map(Node::new).map(|node: Node| {(node.id.to_string(), node)}).collect::<HashMap<_,_>>();
    for node in &nodes {
        println!("{:?}", node);
    }

    let mut steps = 0;
    let mut current = String::from("AAA");

    loop {
        let instruction = &instructions[steps % instructions.len()];
        match instruction {
            Instruction::LEFT => current = nodes[&current].left.to_string(),
            Instruction::RIGHT => current = nodes[&current].right.to_string(),
        }
        steps += 1;

        if current == "ZZZ" {
            break;
        }
    }

    println!("{}", steps);

    let currents: Vec<String> = nodes.keys().filter(|key| {key.ends_with('A')}).map(|x| {x.to_owned()}).collect::<Vec<_>>();
    println!("Starters: {:?}", currents);
    let mut cycles: Vec<i64> = vec![];

    for c in currents {
        let mut steps = 0;
        let mut current = c;
        loop {
            let instruction = &instructions[steps % instructions.len()];
            match instruction {
                Instruction::LEFT => current = nodes[&current].left.to_string(),
                Instruction::RIGHT => current = nodes[&current].right.to_string(),
            }
            steps += 1;

            if current.ends_with('Z') {
                break;
            }
        }

        cycles.push(steps as i64)
    }

    let mut acc = 1;

    for cycle in &cycles {
        acc = lcm(acc, *cycle)
    }

    println!("{}", acc);

}