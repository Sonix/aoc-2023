use aoc_2023::read_lines_from_file;

#[derive(Debug)]
struct Card {
    _id: i32,
    winning: Vec<i32>,
    numbers: Vec<i32>
}

impl Card {
    fn from(line: &str) -> Card {
        let id = line[line.find(' ').unwrap()+1..line.find(':').unwrap()].trim().parse::<i32>().unwrap();
        let winning = line[line.find(':').unwrap()+1..line.find('|').unwrap()].split(' ').filter_map(|str| {match str.trim().parse::<i32>() { Ok(num) => Some(num), _ => None }}).collect::<Vec<_>>();
        let numbers = line[line.find('|').unwrap()+1..].split(' ').filter_map(|str| {match str.trim().parse::<i32>() { Ok(num) => Some(num), _ => None }}).collect::<Vec<_>>();

        Card {
            _id: id,
            winning,
            numbers
        }
    }

    fn value(&self) -> i32 {
        let num = self.matches();
        match num {
            0 => 0,
            i => 2i32.pow((i - 1) as u32)
        }
    }

    fn matches(&self) -> i32 {
        self.numbers.iter().filter(|x| { self.winning.contains(x) }).count() as i32
    }
}

fn main() {
    let games = read_lines_from_file("day4.input").iter().map(|line| {Card::from(line)}).collect::<Vec<_>>();
    println!("Part 1: {}", games.iter().map(|card| {card.value()}).sum::<i32>());

    let mut counts: Vec<i32> = games.iter().map(|_| {1}).collect();
    for i in 0..counts.len() {
        for num in 0..games[i].matches() {
            counts[i + num as usize + 1] += counts[i];
        }
    }

    println!("Part 2: {:?}", counts.iter().sum::<i32>());
}