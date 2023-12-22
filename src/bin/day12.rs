use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use aoc_2023::read_lines_from_file;

#[derive(PartialEq, Clone, Eq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Debug for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Unknown => write!(f, "?").unwrap(),
            Condition::Damaged => write!(f, "#").unwrap(),
            Condition::Operational => write!(f, ".").unwrap(),
        }
    Ok(())
    }
}
impl Condition {
    fn new(input: char) -> Condition {
        match input {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Cant parse {input}")
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Row {
    springs: Vec<Condition>,
    groups: Vec<i32>,
}

impl Debug for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for spring in &self.springs {
            write!(f, "{:?}", spring).unwrap();
        }
        write!(f, " {}", self.groups.iter().map(|x| {x.to_string()}).collect::<Vec<_>>().join(","))
    }
}

impl Row {
    fn new(input: &String) -> Row {
        Row {
            springs: input[0..input.find(' ').unwrap()].chars().map(Condition::new).collect(),
            groups: input[input.find(' ').unwrap()+1..].split(',').filter_map(|x| {x.parse::<i32>().ok()}).collect(),
        }
    }

    fn contiguous_groups(&self) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut counter = 0;
        for spring in &self.springs {
            match spring {
                Condition::Damaged => counter += 1,
                _ => {
                    if counter != 0 {
                        res.push(counter);
                        counter = 0;
                    }
                }
            }
        }
        if counter != 0 {
            res.push(counter);
        }

        res
    }

    fn possible(&self) -> bool {
        let mut res: Vec<i32> = vec![];
        let mut counter = 0;
        let mut idx = 0;
        for spring in &self.springs {
            match spring {
                Condition::Damaged => counter += 1,
                Condition::Operational => {
                    if counter != 0 {
                        if idx < self.groups.len() && counter == self.groups[idx] {
                            res.push(counter);
                            idx += 1;
                            counter = 0;
                        } else {
                            return false;
                        }
                    }
                }
                Condition::Unknown => {
                    if counter != 0 {
                        res.push(counter);
                        counter = 0;
                    }
                    break;
                }
            }
        }
        if counter != 0 {
            if idx < self.groups.len() && counter == self.groups[idx] {
                res.push(counter);
                idx += 1;
            } else {
                return false;
            }
        }

        let unknowns = self.springs.iter().filter(|x| {*x == &Condition::Unknown}).count();

        if idx == self.groups.len() {
            return true;
        } else if idx > self.groups.len() {
            false
        } else {
            unknowns != 0
        }
    }

    fn collapsed(&self) -> bool {
       !self.springs.iter().any(|x| {x == &Condition::Unknown})
    }

    fn valid(&self) -> bool {
        let actual = self.contiguous_groups();
        self.collapsed() && actual == self.groups
    }

    fn remainder(&self) -> Row {
        let mut res: Vec<i32> = vec![];
        let mut counter = 0;
        let mut start = 0;
        let mut idx = 0;
        for spring in &self.springs {
            match spring {
                Condition::Damaged => counter += 1,
                Condition::Operational => {
                    if counter != 0 {
                        if counter == self.groups[idx]{
                            start += counter;
                            idx += 1;
                            res.push(counter);
                            counter = 0;
                        } else {
                            counter = 0;
                            break;
                        }
                    }

                    start += 1;
                }
                Condition::Unknown => {
                    if counter != 0 {
                        counter = 0;
                    }
                    break;
                }
            }
        }

        if counter != 0 {
            if counter == self.groups[idx]{
                start += counter;
                res.push(counter);
            }
        }

        Row{
            springs: self.springs[start as usize..].to_owned(),
            groups: self.groups[res.len()..].to_owned()
        }
    }

    fn variants(&self) -> Vec<Row> {
        if self.collapsed() {
            vec![self.clone()]
        } else {
            let mut res: Vec<Row> = vec![];
            for index in 0..self.springs.len() {
                match self.springs[index] {
                    Condition::Unknown => {
                        let mut variant1 = self.clone();

                        variant1.springs[index] = Condition::Damaged;
                        if variant1.possible() {
                            res.push(variant1);
                        }

                        let mut variant2 = self.clone();
                        variant2.springs[index] = Condition::Operational;
                        if variant2.possible() {
                            res.push(variant2);
                        }

                        break;
                    }

                    _ => {}
                }
            }

            res
        }
    }

    fn possible_variants(&self) -> i64{
        let mut stack: Vec<Row> = Vec::new();
        stack.push(self.clone());
        let mut visited: HashMap<Row, i64> = HashMap::new();

        loop {
            if stack.is_empty() {
                break;
            }
            let cur = stack.pop().unwrap();

            if visited.contains_key(&cur) {
                continue;
            }


            if cur.valid() {
                visited.insert(cur.to_owned(), 1);
            } else if cur.collapsed() {
                visited.insert(cur.to_owned(), 0);
            } else {
                let variants = cur.remainder().variants();
                if variants.iter().all(|row| {visited.contains_key(row)}) {
                    let score = variants.iter().map(|row| {visited[row]}).sum::<i64>();
                    visited.insert(cur.to_owned(), score);
                } else {
                    stack.push(cur.clone());
                    stack.extend(variants.iter().filter(|row| {!visited.contains_key(row)}).map(|x| {x.to_owned()}));
                }
            }
        }

        visited[self]
    }
}
fn main() {
    let lines = read_lines_from_file("day12.input");
    let rows = lines.iter().map(|line| {
        let splits = line.split(' ').collect::<Vec<_>>();
        let springs = [splits[0]; 5].join("?");
        let groups = [splits[1]; 5].join(",");

        format!("{springs} {groups}")
    }).map(|x| {Row::new(&x)}).collect::<Vec<_>>();

    let mut res = 0;
    for row in rows {
        let variants = row.possible_variants();
        res += variants;
        println!("{:?} variants: {}", row, variants);
    }

    println!("{}", res);
}