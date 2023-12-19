use aoc_2023::read_lines_from_file;


fn distance_for_time(charge: i64, time: i64) -> i64 {
    if time - charge < 0 {
        0
    } else {
        (time - charge) * charge
    }
}
fn main() {
    let lines = read_lines_from_file("day6.input");
    let times = lines[0][lines[0].find(':').unwrap()+1..].split(' ').filter_map(|x| {x.trim().parse::<i64>().ok()}).collect::<Vec<_>>();
    let distances = lines[1][lines[1].find(':').unwrap()+1..].split(' ').filter_map(|x| {x.trim().parse::<i64>().ok()}).collect::<Vec<_>>();

    let mut part1 = 1;

    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let mut res = 0i64;

        for charge in 0..time {
            if distance_for_time(charge, time) > distance {
                res += 1;
            }
        }

        part1 *= res;
    }

    println!("{}", part1);

    let time = lines[0][lines[0].find(':').unwrap()+1..].split(' ').map(|x| {x.trim()}).collect::<String>().parse::<i64>().unwrap();
    let distance = lines[1][lines[1].find(':').unwrap()+1..].split(' ').map(|x| {x.trim()}).collect::<String>().parse::<i64>().unwrap();

    let mut lower = 0;
    let mut upper = 0;

    for charge in 0..time {
        if distance_for_time(charge, time) > distance {
            lower = charge;
            break;
        }
    }

    for charge in (0..time).rev() {
        if distance_for_time(charge, time) > distance {
            upper = charge;
            break;
        }
    }

    println!("{}", upper - lower + 1);
}