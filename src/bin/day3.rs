use std::cmp::{max, min};
use aoc_2023::read_lines_from_file;

trait At<T> {
    fn at(&self, row: i32, col: i32) -> T;
}


impl At<char> for Vec<String> {
    fn at(&self, row: i32, col: i32) -> char {
        self.iter().nth(row as usize).unwrap().chars().nth(col as usize).unwrap()
    }
}

impl At<Option<Number>> for Vec<Number> {
    fn at(&self, row: i32, col: i32) -> Option<Number> {
        self.iter().find(|num| {num.inside(row, col)}).map(|t| {t.clone()})
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    row: i32,
    col: i32
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Number {
    number: i32,
    pos: Pos,
    len: i32
}

impl Number {
    fn inside(&self, row: i32, col: i32) -> bool {
        if self.pos.row != row {
            return false
        }

        if col < self.pos.col {
            return false
        }

        if col >= self.pos.col + self.len {
            return false
        }

        true
    }
}

#[derive(Debug)]
struct Gear {
    first: Number,
    second: Number
}

impl Gear {
    fn ratio(&self) -> i32 {
        &self.first.number * &self.second.number
    }
}

struct Schema {
    numbers: Vec<Number>,
    gears: Vec<Gear>
}

impl Schema {
    fn from(input: Vec<String>) -> Schema {
        let numbers = Schema::part_numbers(&input);
        Schema{
            numbers: numbers.clone(),
            gears: Schema::gears(&input, &numbers)
        }
    }

    fn part_numbers(input: &Vec<String>) -> Vec<Number> {
        let mut numbers: Vec<Number> = vec![];

        for row in 0..input.len() {
            let line = &input[row];
            for col in 0..line.len() {
                let char = line.chars().nth(col).unwrap();
                if char.is_digit(10) && !numbers.iter().any(|num| {num.inside(row as i32, col as i32)}){
                    let length = line[col..].chars().position(|c| {!c.is_digit(10)}).unwrap_or_else(||{line.len()-col});
                    let num = Number {
                        number: line[col..col+length].parse().unwrap(),
                        len: length as i32,
                        pos: Pos{row: row as i32,col: col as i32}
                    };
                    if Schema::has_adjacent_part(&input, &num) {
                        numbers.push(num);
                    }
                }
            }
        }

        numbers
    }

    fn has_adjacent_part(input: &Vec<String>,num: &Number) -> bool {
        let pos = &num.pos;

        if pos.col > 0 {
            let char = input.at(pos.row, pos.col-1);
            if !char.is_digit(10) && char != '.' {
                return true
            }
        }

        if pos.col+num.len+1 < input[0].len() as i32 {
            let char = input.at(pos.row, pos.col + num.len);
            if !char.is_digit(10) && char != '.' {
                return true
            }
        }

        if pos.row+1 < input.len() as i32 {
            for idx in max(pos.col-1,0)..min(pos.col+num.len+1, input[0].len() as i32) {
                let char = input.at(pos.row+1, idx);
                if !char.is_digit(10) && char != '.' {
                    return true
                }
            }
        }

        if pos.row > 0 {
            for idx in max(pos.col-1, 0)..min(pos.col+num.len+1, input[0].len() as i32) {
                let char = input.at(pos.row-1, idx);
                if !char.is_digit(10) && char != '.' {
                    return true
                }
            }
        }

        false
    }

    fn gears(input: &Vec<String>, numbers: &Vec<Number>) -> Vec<Gear> {
        let mut gears: Vec<Gear> = vec![];

        for row in 0..input.len() {
            let line = &input[row];
            for col in 0..line.len() {
                let char = line.chars().nth(col).unwrap();
                if char == '*' {
                    let mut adjacent: Vec<Number>  = vec![];
                    for r in max(0, row-1)..min(row+1, input.len())+1 {
                        for c in max(0, col-1)..min(col+1, line.len())+1 {
                            match numbers.at(r as i32, c as i32) {
                                Some(i) => {
                                    if !adjacent.iter().any(|num| {num == &i}) {
                                        adjacent.push(i);
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                    if adjacent.len() == 2 {
                        let gear = Gear {
                            first: adjacent[0].to_owned(),
                            second: adjacent[1].to_owned()
                        };
                        gears.push(gear);
                    }
                }
            }
        }

        gears
    }
}

fn main() {
    let schema = Schema::from(read_lines_from_file("day3.input"));
    println!("{:?}", schema.numbers.iter().map(|x| {x.number}).sum::<i32>());
    println!("{:?}", schema.gears.iter().map(|gear| {gear.ratio()}).sum::<i32>());
}