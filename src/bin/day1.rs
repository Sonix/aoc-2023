use aoc_2023::read_lines_from_file;

fn main() {
    let lines = read_lines_from_file("day1.input");
    for line in &lines {
        println!("{}", number_from_line(&line));
    }

    let numbers = lines.iter().map(|x| {number_from_line(x)}).collect::<Vec<_>>();
    println!("{}", numbers.iter().sum::<u32>());
}

fn number_from_line(line: &str) -> u32 {
    let mut numbers: Vec<u32> = vec![];

    for idx in 0..line.len() {
        match read_digit(&line[idx..]) {
            Some(res) => numbers.push(res),
            None => {}
        }
    }

    let first = numbers.iter().next().unwrap_or_else(|| {panic!("No digit found")});
    let last = numbers.iter().rev().next().unwrap_or_else(|| {panic!("No digit found")});
    return first * 10 + last;
}

fn read_digit(input: &str) -> Option<u32> {
    match input {
        input if input.chars().next().unwrap().to_digit(10).is_some() => Some(input.chars().next().unwrap().to_digit(10).unwrap()),
        input if input.starts_with("one") => Some(1),
        input if input.starts_with("two") => Some(2),
        input if input.starts_with("three") => Some(3),
        input if input.starts_with("four") => Some(4),
        input if input.starts_with("five") => Some(5),
        input if input.starts_with("six") => Some(6),
        input if input.starts_with("seven") => Some(7),
        input if input.starts_with("eight") => Some(8),
        input if input.starts_with("nine") => Some(9),
        _ => None
    }
}