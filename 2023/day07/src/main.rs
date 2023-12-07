use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../in.txt");

    let mut hands = input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(hand, bid)| (parse_hand(hand), bid.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.0.cmp(&b.0));
    let mut res = 0;
    for (i, (hand, bid)) in hands.iter().enumerate() {
        let rank = i + 1;
        println!("{hand:?} {bid}");
        res += bid * rank as i64;
    }
    println!("{res}");
}

fn parse_hand(s: &str) -> Vec<Card> {
    s.chars()
        .map(|c| match c {
            '2'..='9' => Card::C(c as u8 - '0' as u8),
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!(),
        })
        .collect()
}

#[derive(PartialEq)]
struct Hand(Vec<Card>);

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut handset: HashMap<_, i64> = HashMap::new();
        for c in self.0.iter() {
            *handset.entry(c).or_default() += 1;
        }

        let mut sorted = handset.values().copied().collect::<Vec<_>>();
        sorted.sort();

        if &sorted == &[5] {
            return HandType::FiveKind;
        }

        if &sorted == &[1, 4] {
            return HandType::FourKind;
        }

        if sorted == &[2, 3] {
            return HandType::FullHouse;
        }

        if sorted == &[1, 1, 3] {
            return HandType::ThreeKind;
        }

        if sorted == &[1, 2, 2] {
            return HandType::TwoPair;
        }

        if sorted == &[1, 1, 1, 2] {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    C(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let h1 = self.hand_type();
        let h2 = other.hand_type();
        if h1 != h2 {
            h1.partial_cmp(&h2)
        } else {
            for (c1, c2) in self.0.iter().zip(other.0.iter()) {
                if c1 != c2 {
                    return c1.partial_cmp(&c2);
                }
            }
            Some(Ordering::Equal)
        }
    }
}
