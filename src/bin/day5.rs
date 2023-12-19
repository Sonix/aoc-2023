use std::cmp::{max, min};
use std::ops::Range;
use aoc_2023::read_lines_from_file;

#[derive(Default, Debug)]
struct Map {
    name: String,
    rules: Vec<Rule>,
}

impl Map {
    fn new(lines: &[String]) -> Map {
        let mut rules: Vec<Rule> = vec![];
        for line in &lines[1..] {
            rules.push(Rule::new(line))
        }

        Map {name: lines[0].to_owned(), rules }
    }

    fn apply_range(&self, range: Range<i64>) -> Vec<Range<i64>> {
        let mut res: Vec<Range<i64>> = vec![];
        let mut ranges: Vec<Range<i64>> = vec![range];

        loop {
            if ranges.is_empty() {
                break;
            }

            let range = ranges.pop().unwrap();
            let mut matched = false;
            for rule in &self.rules {
                match rule.apply_range(range.to_owned()) {
                    Some((source, dest)) => {
                        res.push(dest);
                        if range.start < source.start {
                            ranges.push(Range{start: range.start, end: source.start});
                        }
                        if source.end < range.end {
                            ranges.push(Range {start: source.end, end: range.end});
                        }

                        matched = true;
                        break;
                    }
                    _ => {}
                }
            }

            if !matched {
                res.push(range);
            }
        }

        res
    }
}

#[derive(Default, Debug)]
struct Rule {
    dest: i64,
    source: i64,
    range: i64,
}

impl Rule {
    fn new(line: &str) -> Rule {
        let nums = line.split(' ').filter_map(|x| { x.trim().parse::<i64>().ok() }).collect::<Vec<_>>();

        Rule { dest: nums[0], source: nums[1], range: nums[2] }
    }

    fn apply_range(&self, range: Range<i64>) -> Option<(Range<i64>, Range<i64>)> {

        let delta = self.dest - self.source;

        if range.end > self.source && range.start <= self.source + self.range - 1{
            let start = max(range.start, self.source);
            let end = min(range.end, self.source + self.range);
            Some((Range { start, end }, Range { start: start + delta, end: end + delta }))
        } else {
            None
        }
    }
}

fn main() {
    let lines = read_lines_from_file("day5.input");
    let seeds: Vec<i64> = lines[0][lines[0].find(':').unwrap() + 1..].split(' ').filter_map(|num| {
        num.trim().parse::<i64>().ok()}).collect();
    let maps = lines[1..].split(|line| { line.is_empty() }).filter(|x| { !x.is_empty() }).map(|maplines| { Map::new(maplines) }).collect::<Vec<_>>();

    let mut part1: Vec<Range<i64>> = vec![];
    for seed in &seeds {
        let mut current: Vec<Range<i64>> = vec![Range {start: *seed, end: *seed}];
        for map in &maps {
            let mut res: Vec<Range<i64>> = vec![];
            for cur in &current {
                res.append(&mut map.apply_range(cur.to_owned()));
            }

            current = res;
        }

        part1.append(&mut current);
    }

    part1.sort_by_key(|x1| {x1.start});
    println!("{:?}", part1.first().unwrap().start);

    let mut part2: Vec<Range<i64>> = vec![];
    for seedgroup in (&seeds).chunks(2) {
        let mut current: Vec<Range<i64>> = vec![Range{start: seedgroup[0], end: seedgroup[0] + seedgroup[1]}];
        for map in &maps {
            let mut res: Vec<Range<i64>> = vec![];
            for cur in &current {
                res.append(&mut map.apply_range(cur.to_owned()));
            }
            current = res;
        }

        part2.append(&mut current);
    }

    part2.sort_by_key(|x1| {x1.start});
    println!("{:?}", part2.first().unwrap().start);
}