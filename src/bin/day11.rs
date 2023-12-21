use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use aoc_2023::read_lines_from_file;

#[derive(PartialEq, Clone)]
enum Tile {
    Empty,
    Galaxy,
}

impl Tile {
    fn new(input: char) -> Tile {
        match input {
            '.' => Tile::Empty,
            '#' => Tile::Galaxy,
            _ => panic!("Cant parse {input}")
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Galaxy => write!(f, "@"),
        }
    }
}

struct Image {
    map: Vec<Vec<Tile>>,
    galaxies: Vec<(i32, i32)>,
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "map:").unwrap();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                write!(f, "{:?}", self.at(x as i32, y as i32).unwrap()).unwrap();
            }
            writeln!(f).unwrap();
        }
        writeln!(f, "galaxies: {:?}", self.galaxies).unwrap();

        Ok(())
    }
}

impl Image {
    fn new(input: Vec<String>) -> Image {
        let mut galaxies: Vec<(i32, i32)> = vec![];
        let mut map: Vec<Vec<Tile>> = vec![];
        for y in 0..input.len() {
            let mut line: Vec<Tile> = vec![];
            for x in 0..input[y].len() {
                let cur = Tile::new(input[y].chars().nth(x).unwrap());
                if cur == Tile::Galaxy {
                    galaxies.push((x as i32, y as i32));
                }
                line.push(cur);
            }
            map.push(line);
        }

        Image {
            map,
            galaxies,
        }
    }

    fn size(&self) -> (i32, i32) {
        (self.map[0].len() as i32, self.map.len() as i32)
    }

    fn at(&self,x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 || x as usize >= self.map[0].len() || y as usize >= self.map.len() {
            None
        } else {
            Some(self.map[y as usize][x as usize].to_owned())
        }
    }

    fn expanded_galaxies(&self, expansion: i32) -> Vec<(i32,i32)> {
        let mut empty_rows: HashSet<i32> = HashSet::new();
        for y in 0..self.size().1 {
            let mut empty = true;
            for x in 0..self.size().0{
                match self.at(x, y) {
                    Some(Tile::Galaxy) => {empty = false; break;},
                    _ => {}
                }
            }
            if empty {
                empty_rows.insert(y);
            }
        }

        let mut empty_cols: HashSet<i32> = HashSet::new();
        for x in 0..self.size().0 {
            let mut empty = true;
            for y in 0..self.size().1{
                match self.at(x, y) {
                    Some(Tile::Galaxy) => {empty = false; break;},
                    _ => {}
                }
            }
            if empty {
                empty_cols.insert(x);
            }
        }

        let mut res: Vec<(i32, i32)> = vec![];
        let mut expanded_rows = 0;

        for y in 0..self.size().1 {
            let mut expanded_cols = 0;
            for x in 0..self.size().0 {
                if empty_cols.contains(&x) {
                    expanded_cols += 1;
                }
                match self.at(x, y) {
                    Some(Tile::Galaxy) => res.push((x + (expanded_cols * (expansion - 1)), y + (expanded_rows * (expansion - 1)))),
                    _ => {}
                }
            }
            if empty_rows.contains(&y) {
                expanded_rows += 1;
            }
        }

        res
    }
}

fn distance(first: (i32, i32), second: (i32, i32)) -> i32 {
    (first.0 - second.0).abs() + (first.1 - second.1).abs()
}

fn main() {
    let lines = read_lines_from_file("day11.input");
    let image = Image::new(lines);
    println!("{image:?}");

    let mut distances: Vec<i64> = vec![];
    let galaxies = image.expanded_galaxies(1_000_000);

    for first in 0..galaxies.len() {
        for second in first + 1..galaxies.len() {
            distances.push(distance(galaxies[first], galaxies[second]) as i64);
        }
    }

    println!("{}", distances.iter().sum::<i64>())
}