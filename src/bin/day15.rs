use std::array::from_fn;
use std::collections::HashMap;
use aoc_2023::read_lines_from_file;

fn hash(input: &str) -> i32 {
    let mut res = 0;
    for byte in input.as_bytes() {
        res = ((res + *byte as i32) * 17) % 256;
    }

    res
}

#[derive(Clone, Debug)]
struct Box {
    lenses: Vec<i32>,
    index: HashMap<String, i32>,
}

impl Box {
    fn new() -> Box {
        Box {
            lenses: vec![],
            index: HashMap::new(),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Remove,
    Add,
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '=' => Operation::Add,
            '-' => Operation::Remove,
            _ => panic!("Cant parse {value}"),
        }
    }
}

#[derive(Debug)]
struct Step {
    label: String,
    op: Operation,
    length: Option<i32>,
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let idx_op = value.find(|x| { x == '=' || x == '-' }).unwrap();
        let label = value[0..idx_op].to_owned();
        let op = Operation::from(value.chars().nth(idx_op).unwrap());
        let length = match op {
            Operation::Remove => None,
            Operation::Add => Some(value[idx_op+1..].parse().unwrap()),
        };

        Step {
            label,
            op,
            length,
        }
    }
}

fn main() {
    let line = read_lines_from_file("day15.input")[0].to_owned();
    let steps = line.split(',').map(Step::from).collect::<Vec<_>>();
    let mut boxes: [Box; 256] = from_fn(|_| {Box::new()});

    for step in steps {
        let target = hash(&step.label);

        match &step.op {
            Operation::Remove => {
                if boxes[target as usize].index.contains_key(&step.label) {
                    let index = boxes[target as usize].index[&step.label];
                    boxes[target as usize].lenses.remove(index as usize);
                    boxes[target as usize].index.remove(&step.label);
                    boxes[target as usize].index = boxes[target as usize].index.iter().map(|(label,idx)| { if idx > &index {(label.to_owned(), idx - 1)} else {(label.to_owned(), *idx)} }).collect();
                }
            }
            Operation::Add => {
                if boxes[target as usize].index.contains_key(&step.label) {
                    let index = boxes[target as usize].index[&step.label];
                    boxes[target as usize].lenses[index as usize] = step.length.unwrap();
                } else {
                    boxes[target as usize].lenses.push(step.length.unwrap());
                    boxes[target as usize].index.insert(step.label,boxes[target as usize].lenses.len() as i32 - 1);
                }
            }
        }
    }

    let res = boxes.iter().enumerate().map(|(box_index, b)| {
        b.lenses.iter().enumerate().map(|(lens_index, length)| {
            length * (lens_index as i32 + 1) * (box_index as i32 + 1)
        }).sum::<i32>()
    }).sum::<i32>();

    println!("Res: {}", res);
}