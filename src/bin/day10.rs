use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use aoc_2023::read_lines_from_file;

#[derive(Debug, Clone)]
#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East
        }
    }
}
#[derive(Clone, PartialEq)]
enum Tile {
    Ground,
    Pipe(Direction, Direction),
    Start,
}

impl Tile {
    fn new(input: char) -> Self {
        match input {
            '|' => Tile::Pipe(Direction::North,Direction::South),
            '-' => Tile::Pipe(Direction::East,Direction::West),
            'F' => Tile::Pipe(Direction::South,Direction::East),
            '7' => Tile::Pipe(Direction::West,Direction::South),
            'L' => Tile::Pipe(Direction::East,Direction::North),
            'J' => Tile::Pipe(Direction::West,Direction::North),
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Cant parse {}", input)
        }
    }

    fn connects(&self, dir: &Direction) -> bool {
        match self {
            Tile::Ground => false,
            Tile::Start => true,
            Tile::Pipe(first, second) => {
               match dir {
                   Direction::North => first == &Direction::North || second == &Direction::North,
                   Direction::South => first == &Direction::South || second == &Direction::South,
                   Direction::East => first == &Direction::East || second == &Direction::East,
                   Direction::West => first == &Direction::West || second == &Direction::West,
               }
            }
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ground => write!(f, "."),
            Tile::Start => write!(f, "S"),
            Tile::Pipe(first, second) => {
                match (first, second) {
                    (Direction::North, Direction::South) => write!(f, "┃"),
                    (Direction::East, Direction::West) => write!(f, "━"),
                    (Direction::South, Direction::East) => write!(f, "┏"),
                    (Direction::West, Direction::South) => write!(f, "┓"),
                    (Direction::East, Direction::North) => write!(f, "┗"),
                    (Direction::West, Direction::North) => write!(f, "┛"),
                    _ => panic!()
                }
            }
        }
    }
}

struct Map {
    map: Vec<Vec<Tile>>,
    start: (i32, i32),
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Start: {:?}", self.start).unwrap();

        for line in &self.map {
            for tile in line {
                write!(f, "{:?}", tile).unwrap()
            }
            write!(f, "\n").unwrap()
        }
        Ok(())
    }
}

impl Map {
    fn new(input: Vec<String>) -> Map {
        let map = input.iter().map(|line| {line.chars().map(Tile::new).collect::<Vec<_>>()}).collect::<Vec<_>>();
        let start = ( *map.iter().flat_map(|row| {row.iter().position(|tile| {tile == &Tile::Start})}).collect::<Vec<_>>().first().unwrap() as i32 ,map.iter().position(|row| {row.contains(&Tile::Start)}).unwrap() as i32);

        Map {
            map,
            start
        }
    }

    fn at(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || x as usize >= self.map.first().unwrap().len() || y < 0 || y as usize >= self.map.len() {
            None
        } else {
            Some(self.map[y as usize][x as usize].to_owned())
        }
    }

    fn size(&self) -> (i32, i32) {
        (self.map.first().unwrap().len() as i32,self.map.len() as i32)
    }

    fn connects_to(&self, pos: (i32, i32), dir: Direction) -> bool {
        match self.at(pos.0, pos.1) {
            Some(tile) => {
                match tile.connects(&dir) {
                    true => {
                        match dir {
                            Direction::North => match self.at(pos.0, pos.1 - 1) {None => false, Some(other) => other.connects(&dir.opposite())},
                            Direction::South => match self.at(pos.0, pos.1 + 1) {None => false, Some(other) => other.connects(&dir.opposite())},
                            Direction::East => match self.at(pos.0 + 1, pos.1) {None => false, Some(other) => other.connects(&dir.opposite())},
                            Direction::West => match self.at(pos.0 - 1, pos.1) {None => false, Some(other) => other.connects(&dir.opposite())},
                        }
                    },
                    false => false,
                }
            },
            None => panic!(),
        }
    }

    fn inside(&self, path: &HashSet<(i32, i32)>, pos: (i32, i32)) -> bool {
        let mut east_count = 0;
        let mut entry_direction = None;
        for x in 0..pos.0 {
            if path.contains(&(x, pos.1)) {
               match self.at(x, pos.1).unwrap() {
                   Tile::Pipe(first, second) if first == Direction::North && second == Direction::South => east_count += 1,
                   Tile::Pipe(first, second) if first == Direction::North || first == Direction::South || second == Direction::North || second == Direction::South => {
                       let mut dir;
                       if first == Direction::North || second == Direction::North {
                           dir = Direction::North;
                       } else {
                           dir = Direction::South;
                       }

                       match entry_direction {
                           Some(ref previous) => {
                               match previous {
                                   prev if prev != &dir =>  {
                                       east_count += 1;
                                       entry_direction = None;
                                   },
                                   _ => entry_direction = None
                               }
                           },
                           _ => {
                               entry_direction = Some(dir);
                           }
                       }
                   },
                   _ => {}
               }
            }
        }

        east_count % 2 != 0
    }

    fn print_inside(&self, path: &HashSet<(i32, i32)>) -> i32 {
        let mut res = 0;
        for y in 0..self.size().1 {
            for x in 0..self.size().0 {
                match self.at(x,y) {
                    Some(t) if path.contains(&(x, y)) => print!("\x1b[1m\x1b[31m{:?}\x1b[0m", t),
                    Some(_) => {
                        match self.inside(path, (x,y)) {
                            true => {res += 1; print!("\x1b[92mX\x1b[0m")},
                            false => print!("\x1b[37mO\x1b[0m")
                        }
                    }
                    _ => panic!()
                }
            }
            print!("\n")
        }

        res
    }
}
fn main() {
    let lines = read_lines_from_file("day10.input");
    let map = Map::new(lines);

    println!("{:?}", map);
    println!("Tile {:?}", map.start);
    println!("North: {}", map.connects_to(map.start, Direction::North));
    println!("South: {}", map.connects_to(map.start, Direction::South));
    println!("East: {}", map.connects_to(map.start, Direction::East));
    println!("West: {}", map.connects_to(map.start, Direction::West));

    let mut result: HashSet<(i32, i32)> = HashSet::new();

    let mut options: Vec<Vec<(i32, i32)>> = vec![vec![map.start]];

    loop {
        let current = options.pop().unwrap();
        let pos = *current.last().unwrap();

        let mut next: Vec<(i32, i32)> = vec![];

        if map.connects_to(pos, Direction::North) {
            next.push((pos.0, pos.1 - 1));
        }

        if map.connects_to(pos, Direction::South) {
            next.push((pos.0, pos.1 + 1));
        }

        if map.connects_to(pos, Direction::East) {
            next.push((pos.0 + 1, pos.1));
        }

        if map.connects_to(pos, Direction::West) {
            next.push((pos.0 - 1, pos.1));
       }

        for next in next {
            if !(current.len() > 1 && current.iter().rev().nth(1).unwrap() == &next) {
                match map.at(next.0, next.1) {
                    Some(Tile::Start) => {
                        result =  current.iter().map(|x| {*x}).collect();
                    },
                    Some(_) => {
                        let mut option = current.clone();
                        option.push(next);
                        options.push(option);
                    }

                    None => panic!()
                }
            }
        }

        if !result.is_empty() {
            break;
        }
    }
    println!("{:?}", result.len() / 2);

    let res = map.print_inside(&result);
    println!("Res: {}", res);

}