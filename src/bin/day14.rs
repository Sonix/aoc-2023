use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use aoc_2023::read_lines_from_file;

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn vector(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
enum Tile {
    Round,
    Cube,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Tile::Round,
            '#' => Tile::Cube,
            '.' => Tile::Empty,
            _ => panic!("Cant parse {value}"),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Round => write!(f, "O").unwrap(),
            Tile::Cube => write!(f, "#").unwrap(),
            Tile::Empty => write!(f, ".").unwrap(),
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl From<&Vec<String>> for Map {
    fn from(value: &Vec<String>) -> Self {
        Map {
            tiles: value.iter().map(|line| {line.chars().map(Tile::from).collect()}).collect()
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.tiles.size().1 {
            for x in 0..self.tiles.size().0 {
                write!(f, "{:?}", self.tiles.at(x as i32, y as i32).unwrap()).unwrap()
            }
            writeln!(f).unwrap();
        }

        Ok(())
    }
}

trait Size {
    fn size(&self) -> (usize, usize);
}

trait At {
    fn at(&self, x: i32, y: i32) -> Option<Tile>;
}

impl Size for Vec<Vec<Tile>> {
    fn size(&self) -> (usize, usize) {
        (self[0].len(), self.len())
    }
}

impl At for Vec<Vec<Tile>> {
    fn at(&self, x: i32, y: i32) -> Option<Tile> {
        if  x < 0 || x >= self.size().0 as i32 || y < 0 || y >= self.size().1 as i32{
            None
        } else {
            Some(self[y as usize][x as usize])
        }
    }
}

impl Map {
    fn tilted(&self, dir: &Direction) -> Map {
        let mut tiles = self.tiles.to_owned();

        match dir {
            Direction::North => {
                for y in 0..self.tiles.size().1 {
                    for x in 0..self.tiles.size().0 {
                        if self.tiles.at(x as i32, y as i32) != Some(Tile::Round) {
                            continue;
                        }

                        let pos = Map::final_position(&tiles, x, y, dir);
                        if pos != (x, y) {
                            tiles[y][x] = Tile::Empty;
                            tiles[pos.1][pos.0] = Tile::Round;
                        }
                    }
                }
            }
            Direction::South => {
                for y in (0..self.tiles.size().1).rev() {
                    for x in 0..self.tiles.size().0 {
                        if self.tiles.at(x as i32, y as i32) != Some(Tile::Round) {
                            continue;
                        }

                        let pos = Map::final_position(&tiles, x, y, dir);
                        if pos != (x, y) {
                            tiles[y][x] = Tile::Empty;
                            tiles[pos.1][pos.0] = Tile::Round;
                        }
                    }
                }
            }
            Direction::East => {
                for x in (0..self.tiles.size().0).rev() {
                    for y in 0..self.tiles.size().1 {
                        if self.tiles.at(x as i32, y as i32) != Some(Tile::Round) {
                            continue;
                        }

                        let pos = Map::final_position(&tiles, x, y, dir);
                        if pos != (x, y) {
                            tiles[y][x] = Tile::Empty;
                            tiles[pos.1][pos.0] = Tile::Round;
                        }
                    }
                }
            }
            Direction::West => {
                for x in 0..self.tiles.size().0 {
                    for y in 0..self.tiles.size().1 {
                        if self.tiles.at(x as i32, y as i32) != Some(Tile::Round) {
                            continue;
                        }

                        let pos = Map::final_position(&tiles, x, y, dir);
                        if pos != (x, y) {
                            tiles[y][x] = Tile::Empty;
                            tiles[pos.1][pos.0] = Tile::Round;
                        }
                    }
                }
            }
        }

        Map{tiles}
    }

    fn circle(&self) -> Map {
        self.tilted(&Direction::North).tilted(&Direction::West).tilted(&Direction::South).tilted(&Direction::East)
    }

    fn score(&self) -> i32 {
        let mut res = 0;
        for y in 0..self.tiles.size().1 {
            for x in 0..self.tiles.size().0 {
                match self.tiles.at(x as i32,y as i32) {
                    None => {}
                    Some(tile) => {
                        match tile {
                            Tile::Round => res += self.tiles.size().1 - y,
                            _ => {}
                        }
                    }
                }
            }
        }

        res as i32
    }

    fn final_position(tiles: &Vec<Vec<Tile>>, x: usize, y: usize, dir: &Direction) -> (usize, usize) {

        let mut current = (x as i32, y as i32);
        loop {
            let next = (current.0 + dir.vector().0, current.1 + dir.vector().1);
            match tiles.at(next.0, next.1) {
                None => break,
                Some(tile) => {
                    match tile {
                        Tile::Empty => current = next,
                        _ => break,
                    }
                }
            }
        }

        (current.0 as usize, current.1 as usize)
    }
}

fn main() {
    let lines = read_lines_from_file("day14.input");
    let map = Map::from(&lines);

    let tilted = map.tilted(&Direction::North);
    println!("Score: {}", tilted.score());

    let mut seen: HashMap<Map, i32> = HashMap::new();
    let mut indexed: HashMap<i32, Map> = HashMap::new();
    let mut circle = map.circle();
    let mut count = 1;
    let start;
    seen.insert(circle.clone(), count);
    indexed.insert(count, circle.clone());

    loop {
        let next = circle.circle();
        count += 1;

        if seen.contains_key(&next) {
            println!("Cycle: {count}");
            start = seen[&next];
            println!("First seen: {}", seen[&next]);
            break;
        }

        circle = next;
        seen.insert(circle.clone(), count);
        indexed.insert(count, circle.clone());
    }

    let target = (1_000_000_000 - start) % (count - start) + start;
    println!("Tar: {target} Val: {}",indexed[&target].score());
}