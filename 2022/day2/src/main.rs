use std::str::FromStr;

pub enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Rps {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input = input.trim_end();

    let guide: Vec<(Rps, Rps)> = input
        .split("\n")
        .map(|line| {
            let mut iter = line
                .split_whitespace()
                .map(|item| item.parse::<Rps>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect();

    let mut score = 0;
    for r in &guide {
        let won_points = match r {
            (Rps::Rock, Rps::Paper) | (Rps::Paper, Rps::Scissors) | (Rps::Scissors, Rps::Rock) => 6,
            (Rps::Rock, Rps::Scissors) | (Rps::Paper, Rps::Rock) | (Rps::Scissors, Rps::Paper) => 0,
            (Rps::Paper, Rps::Paper) | (Rps::Rock, Rps::Rock) | (Rps::Scissors, Rps::Scissors) => 3,
        };
        let chosen_points = match r.1 {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        };
        score += won_points + chosen_points;
    }

    let mut new_score = 0;
    for r in guide {
        let won_points = match r.1 {
            Rps::Rock => 0,
            Rps::Paper => 3,
            Rps::Scissors => 6,
        };
        let chosen_hand = match r {
            (Rps::Rock, Rps::Rock) => Rps::Scissors,
            (Rps::Rock, Rps::Paper) => Rps::Rock,
            (Rps::Rock, Rps::Scissors) => Rps::Paper,
            (Rps::Paper, Rps::Rock) => Rps::Rock,
            (Rps::Paper, Rps::Paper) => Rps::Paper,
            (Rps::Paper, Rps::Scissors) => Rps::Scissors,
            (Rps::Scissors, Rps::Rock) => Rps::Paper,
            (Rps::Scissors, Rps::Paper) => Rps::Scissors,
            (Rps::Scissors, Rps::Scissors) => Rps::Rock,
        };
        let chosen_points = match chosen_hand {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        };
        new_score += won_points + chosen_points;
    }
    println!("{score} {new_score}");
}
