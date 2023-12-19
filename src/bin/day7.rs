use std::cmp::Ordering;
use std::collections::HashMap;
use aoc_2023::read_lines_from_file;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Num(i32),
    J,
    Q,
    K,
    A,
}

impl Card {
    fn new(input: &str) -> Card {
        match input {
            "A" => Card::A,
            "K" => Card::K,
            "Q" => Card::Q,
            "J" => Card::J,
            "T" => Card::Num(10),
            input if input.parse::<i32>().is_ok() => Card::Num(input.parse::<i32>().unwrap()),
            _ => panic!("Cant parse {}", input)
        }
    }

    fn compare_joker(&self, other: &Self) -> Option<Ordering> {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(res) => {
                if *self == Card::J {
                    Some(Ordering::Less)
                } else if *other == Card::J {
                    Some(Ordering::Greater)
                } else {
                    Some(res)
                }
            }
            None => None,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Type {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord)]
struct Hand {
    cards: [Card; 5],
    bid: i32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.iter().eq(&other.cards)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.kind().partial_cmp(&other.kind()) {
            Some(Ordering::Equal) => {
                for idx in 0..5 {
                    match self.cards[idx].partial_cmp(&other.cards[idx]) {
                        Some(Ordering::Equal) => {}
                        Some(res) => {
                            return Some(res)
                        }
                        None => { }
                    }
                }

                Some(Ordering::Equal)
            },
            Some(res) => Some(res),
            None => None
        }
    }
}

impl Hand {
    fn new(line: &str) -> Hand {
        let cards = [Card::new(&line[0..1]), Card::new(&line[1..2]), Card::new(&line[2..3]), Card::new(&line[3..4]), Card::new(&line[4..5])];
        let bid = line[line.find(' ').unwrap()+1..].trim().parse::<i32>().unwrap();
        Hand {cards, bid}
    }

    fn kind(&self) -> Type {
        let mut sorted = HashMap::new();
        for card in &self.cards {
            if sorted.contains_key(card) {
                sorted.insert(card, sorted[card] + 1);
            } else {
                sorted.insert(card, 1);
            }
        }

        let mut values = sorted.values().collect::<Vec<_>>();
        values.sort();
        values.reverse();

        match values {
            sorted if sorted.len() == 1 && *sorted[0] == 5 => Type::FiveOfAKind,
            sorted if sorted.len() == 2 && *sorted[0] == 4 => Type::FourOfAKind,
            sorted if sorted.len() == 2 && *sorted[0] == 3 && *sorted[1] == 2 => Type::FullHouse,
            sorted if sorted.len() >= 2 && *sorted[0] == 3 => Type::ThreeOfAKind,
            sorted if sorted.len() >= 2 && *sorted[0] == 2 && *sorted[1] == 2 => Type::TwoPair,
            sorted if sorted.len() >= 2 && *sorted[0] == 2 => Type::Pair,
            _ => Type::HighCard,
        }
    }

    fn joker_kind(&self) -> Type {
        let mut sorted = HashMap::new();
        for card in &self.cards {
            if sorted.contains_key(card) {
                sorted.insert(card, sorted[card] + 1);
            } else {
                sorted.insert(card, 1);
            }
        }

        let mut jokers = 0;
        if sorted.contains_key(&Card::J) {
            jokers = sorted[&Card::J];
        }

        sorted.remove(&Card::J);

        let mut values = sorted.values().collect::<Vec<_>>();
        values.sort();
        values.reverse();

        match values {
            sorted if sorted.len() >= 1 && *sorted[0] + jokers >= 5 => Type::FiveOfAKind,
            sorted if sorted.len() >= 1 && *sorted[0] + jokers >= 4 => Type::FourOfAKind,
            sorted if sorted.len() >= 2 && *sorted[0] + jokers >= 3 && *sorted[1] + jokers >= 2 && (3-sorted[0]) + (2-sorted[1]) <= jokers => Type::FullHouse,
            sorted if sorted.len() >= 1 && *sorted[0] + jokers >= 3 => Type::ThreeOfAKind,
            sorted if sorted.len() >= 2 && *sorted[0] + jokers >= 2 && *sorted[1] + jokers >= 2 && (2-sorted[0]) + (2-sorted[1]) <= jokers  => Type::TwoPair,
            sorted if sorted.len() >= 1 && *sorted[0] + jokers >= 2 => Type::Pair,
            _ if jokers == 5 => Type::FiveOfAKind,
            _ => Type::HighCard,
        }
    }

    fn compare_joker(&self, other: &Self) -> Ordering {
        match self.joker_kind().partial_cmp(&other.joker_kind()) {
            Some(Ordering::Equal) => {
                for idx in 0..5 {
                    match self.cards[idx].compare_joker(&other.cards[idx]) {
                        Some(Ordering::Equal) => {}
                        Some(res) => {
                            return res
                        }
                        None => { }
                    }
                }

                Ordering::Equal
            },
            Some(res) => res,
            None => panic!()
        }
    }
}

fn main() {
    let lines = read_lines_from_file("day7.input");
    let mut hands = lines.iter().map(|x| {Hand::new(x)}).collect::<Vec<_>>();

    hands.sort();
    let mut res = 0;

    for idx in 0..hands.len() {
        res += hands[idx].bid * (idx as i32 + 1);
    }

    println!("{}", res);

    hands.sort_by(Hand::compare_joker);
    res = 0;

    for idx in 0..hands.len() {
        res += hands[idx].bid * (idx as i32 + 1);
    }

    println!("{}", res);
}