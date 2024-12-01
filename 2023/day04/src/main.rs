use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();
    doit(&input);
}

fn parse(card: &str) -> HashSet<i64> {
    card.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn doit(input: &str) {
    let mut res1 = 0;
    let mut res2 = 0;
    let mut won_cards = HashMap::new();
    for (card_no, card) in input.lines().enumerate() {
        let (_header, card) = card.split_once(':').unwrap();
        let (numbers, winners) = card.split_once('|').unwrap();
        let numbers = parse(numbers);
        let winners = parse(winners);

        let count = numbers.intersection(&winners).count() as u32;
        if count > 0 {
            res1 += 2_i64.pow(count - 1)
        };

        let card_no = (card_no + 1) as u32;
        let current_cards = won_cards.get(&card_no).unwrap_or(&0) + 1;
        if count > 0 {
            for n in card_no + 1..card_no + count + 1 {
                *won_cards.entry(n).or_default() += current_cards;
            }
        }
        res2 += current_cards;
    }

    println!("{res1}");
    println!("{res2}");
}

#[cfg(test)]
mod test {
    use crate::doit;

    #[test]
    fn test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        doit(input);
    }
}
