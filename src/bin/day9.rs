use aoc_2023::read_lines_from_file;

#[derive(Debug)]
struct Series {
    values: Vec<i32>,
}

impl Series {
    fn new(line: &String) -> Series {
        Series {
            values: line.split(' ').map(|s| {s.trim()}).filter(|s| {!s.is_empty()}).map(|s| {s.parse::<i32>().unwrap()}).collect()
        }
    }

    fn next(&self) -> (i32, i32) {

        let mut stack: Vec<Vec<i32>> = vec![self.values.clone()];
        let mut current = &stack[0];

        loop {
            let mut res: Vec<i32> = vec![];

            for pair in current.windows(2) {
                if pair.len() == 1 {
                    break;
                }

                res.push(pair[1] - pair[0]);
            }

            if res.iter().all(|x| {*x == 0}) {
                break;
            }

            stack.push(res);
            current = stack.last().unwrap();
        }

        let mut right = 0;

        for value in stack.iter().map(|v| {v.last().unwrap()}).rev() {
            right = value + right;
        }

        let mut left = 0;

        for value in stack.iter().map(|v| {v.first().unwrap()}).rev() {
            left = value - left;
        }

        (left, right)
    }
}

fn main() {
    let lines = read_lines_from_file("day9.input");
    let series = lines.iter().map(Series::new).collect::<Vec<_>>();

    for series in &series {
        println!("{:?} {:?}", series, series.next())
    }

    let left = series.iter().map(|x| {x.next().0}).sum::<i32>();
    let right = series.iter().map(|x| {x.next().1}).sum::<i32>();

    println!("Left: {}", left);
    println!("Right: {}", right)
}