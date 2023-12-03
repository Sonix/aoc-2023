use aoc_2023::read_lines_from_file;

#[derive(Debug)]
struct Set {
    red: i32,
    blue: i32,
    green: i32
}

impl Set {
    fn from(input: &str) -> Set {
        let groups = input.split(',').map(|s| {s.trim()}).collect::<Vec<_>>();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for group in groups {
            let count: i32 = group[..group.find(' ').unwrap()].parse().unwrap();

            match group.chars().nth(group.find(' ').unwrap()+1).unwrap() {
                'r' => red = count,
                'b' => blue = count,
                'g' => green = count,
                _ => panic!("Unexpected char")
            }
        }

        Set{
            blue,
            green,
            red
        }
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>
}

impl Game {
    fn from(input: &str) -> Game {
        let id = input[5..input.find(':').unwrap()].parse().unwrap();
        let sets = input[input.find(':').unwrap() + 1..].split(";").map(|group| {Set::from(group)}).collect();
        Game {
            id,
            sets,
        }
    }

    fn possible(&self, red: i32, green: i32, blue: i32) -> bool {
        !self.sets.iter().any(|set| {set.red > red || set.blue > blue || set.green > green})
    }

    fn power(&self) -> i32 {
        let red = self.sets.iter().map(|set| {set.red}).max().unwrap();
        let blue = self.sets.iter().map(|set| {set.blue}).max().unwrap();
        let green = self.sets.iter().map(|set| {set.green}).max().unwrap();
         red * blue * green
    }
}
fn main() {
    let lines = read_lines_from_file("day2.input");
    let games = lines.iter().map(|line| {Game::from(&line)}).collect::<Vec<_>>();
    let result: i32 = games.iter().filter_map(|g| {if g.possible(12,13,14) {Some(g.id)} else {None}}).sum();
    let power: i32 = games.iter().map(|game| {game.power()}).sum();

    println!("{}", result);
    println!("{}", power);
}