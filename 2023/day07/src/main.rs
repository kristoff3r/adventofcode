use std::{cmp::Ordering, collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../in.txt");
    // let input = include_str!("../example.txt");

    let mut hands = input
        .trim_end()
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(hand, bid)| (hand.parse::<Hand>().unwrap(), bid.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut res = 0;
    for (i, (hand, bid)) in hands.iter().enumerate() {
        let rank = i + 1;
        println!("{hand:?} {bid}");
        res += bid * rank as i64;
    }
    println!("{res}");
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand(
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
                .collect::<Vec<_>>(),
        ))
    }
}

#[derive(PartialEq, Debug)]
struct Hand(Vec<Card>);

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut handset: HashMap<_, i64> = HashMap::new();
        for c in self.0.iter() {
            *handset.entry(c).or_default() += 1;
        }

        let joker_count = handset.remove(&Card::J).unwrap_or(0);

        let mut sorted = handset.values().copied().collect::<Vec<_>>();
        sorted.sort();
        let max = sorted.iter().max().copied().unwrap_or(0);

        if max + joker_count == 5 {
            return HandType::FiveKind;
        }

        if max + joker_count == 4 {
            return HandType::FourKind;
        }

        if sorted == &[2, 3] || sorted == &[2, 2] {
            return HandType::FullHouse;
        }

        if max + joker_count == 3 {
            return HandType::ThreeKind;
        }

        if sorted == &[1, 2, 2] {
            return HandType::TwoPair;
        }

        if sorted == &[1, 1, 1, 2] || sorted == &[1, 1, 1, 1] {
            return HandType::OnePair;
        }

        assert_eq!(joker_count, 0);

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
    J,
    C(u8),
    T,
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
