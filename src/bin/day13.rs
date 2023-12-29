use aoc_2023::read_lines_from_file;


#[derive(Debug, PartialEq, Copy, Clone)]
enum Reflection {
    Horizontal(i32),
    Vertical(i32),
}

impl Reflection {
    fn value(&self) -> i32 {
        match self {
            Reflection::Horizontal(i) => i * 100,
            Reflection::Vertical(i) => i * 1,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn new(input: char) -> Tile {
        match input {
            '.' => Tile::Ash,
            '#' => Tile::Rock,
            _ => panic!("Cant parse {input}")
        }
    }

    fn flipped(&self) -> Tile {
        match self {
            Tile::Ash => Tile::Rock,
            Tile::Rock => Tile::Ash,
        }
    }
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<Tile>>,
}

impl Pattern {
    fn new(input: &[String]) -> Pattern {
        Pattern {
            rows: input.iter().map(|x| {x.chars().map(Tile::new).collect()}).collect()
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.rows[0].len(), self.rows.len())
    }

    fn row(&self, index: usize) -> Option<Vec<Tile>> {
        if index < 1 || index > self.size().1 {
            None
        } else {
            Some(self.rows[index - 1].clone())
        }
    }

    fn col(&self, index: usize) -> Option<Vec<Tile>> {
        if index < 1 || index > self.size().0 {
            None
        } else {
            let mut res: Vec<Tile> = vec![];
            for idx in 0..self.size().1 {
                res.push(self.rows[idx][index - 1])
            }
            Some(res)
        }
    }

    fn horizontal_reflections(&self) -> Vec<usize> {
        let mut res = vec![];
        for split in 1..self.size().1 {
            let mut matches = true;
            for dist in 0..split {
                let top = self.row(split-dist);
                let bot = self.row(split+dist + 1);
                if !Pattern::reflects(&top, &bot) {
                    matches = false;
                    break;
                }
            }

            if matches {
                res.push(split);
            }
        }

        res
    }

    fn vertical_reflections(&self) -> Vec<usize> {
        let mut res = vec![];
        for split in 1..self.size().0 {
            let mut matches = true;
            for dist in 0..split {
                let left = self.col(split-dist);
                let right = self.col(split+dist + 1);
                if !Pattern::reflects(&left, &right) {
                    matches = false;
                    break;
                }
            }

            if matches {
                res.push(split);
            }
        }

        res
    }

    fn reflects(first: &Option<Vec<Tile>>, second: &Option<Vec<Tile>>) -> bool {
        match first {
            None => true,
            Some(first) => {
                match second {
                    None => true,
                    Some(second) => {
                        for idx in 0..first.len() {
                            if first[idx] != second[idx] {
                                return false;
                            }
                        }

                        true
                    }
                }
            }
        }
    }

    fn reflections(&self) -> Vec<Reflection> {
        self.horizontal_reflections().iter().map(|x| {Reflection::Horizontal(*x as i32)}).chain(self.vertical_reflections().iter().map(|x| {Reflection::Vertical(*x as i32)})).collect()
    }

    fn repaired_at(&self, x: usize, y: usize) -> Pattern {
        let mut rows = self.rows.to_owned();
        rows[y - 1][x - 1] = rows[y - 1][x - 1].flipped();
        Pattern {
            rows
        }
    }

    fn with_alternate_reflection(&self) -> Option<(Pattern, Reflection)> {
        let current = self.reflections();
        for y in 1..=self.size().1 {
            for x in 1..=self.size().0 {
                let res = self.repaired_at(x, y);
                let reflections = res.reflections();
                if reflections != current && reflections.iter().any(|reflection| {!current.contains(reflection)}) {
                    let alternate = reflections.iter().filter(|reflection| {!current.contains(reflection)}).last().unwrap().to_owned();
                    return Some((res, alternate));
                }
            }
        }

        None
    }
}


fn main() {
    let lines = read_lines_from_file("day13.input");
    let patterns = lines.split(|x| {x.is_empty()}).map(Pattern::new).collect::<Vec<_>>();
    println!("Res: {}", patterns.iter().flat_map(Pattern::reflections).map(|reflection| {reflection.value()}).sum::<i32>());

    let alternates = patterns.iter().flat_map(Pattern::with_alternate_reflection).map(|(_, reflection)| {reflection}).collect::<Vec<_>>();
    println!("Res 2: {}", alternates.iter().map(|reflection| {reflection.value()}).sum::<i32>());
}