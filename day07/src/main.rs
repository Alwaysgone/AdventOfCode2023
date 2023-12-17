use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::str::FromStr;

fn main() {
    match part01() {
        Ok(total_winnings) => println!("total winnings part01: {}", total_winnings),
        Err(e) => println!("an error occurred in part01: {}", e),
    }

}

#[derive(PartialEq, PartialOrd)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s {
            "C2" => Ok(Card::C2),
            "C3" => Ok(Card::C3),
            "C4" => Ok(Card::C4),
            "C5" => Ok(Card::C5),
            "C6" => Ok(Card::C6),
            "C7" => Ok(Card::C7),
            "C8" => Ok(Card::C8),
            "C9" => Ok(Card::C9),
            "CT" => Ok(Card::CT),
            "CJ" => Ok(Card::CJ),
            "CQ" => Ok(Card::CQ),
            "CK" => Ok(Card::CK),
            "CA" => Ok(Card::CA),
            _ => Err(())
        }
    }
}

struct Hand {
    cards:String,
    bid:u64,
}

impl Hand {
    fn is_stronger(&self, other: &Self) -> bool {
        for cards in self.cards.chars()
        .map(|c| {
            let mut card:String = String::from("C");
            card.push(c);
            Card::from_str(&card).unwrap()
        }).zip(other.cards.chars()
        .map(|c| {
            let mut card:String = String::from("C");
            card.push(c);
            Card::from_str(&card).unwrap()
        })) {
            if cards.0 > cards.1 {
                return true;
            }
            if cards.0 < cards.1 {
                return false;
            }
        }
        // should never happen since all hands are different
        false
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.cards.partial_cmp(&other.cards) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.bid.partial_cmp(&other.bid)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

fn part01() -> Result<u64>{
    let path = Path::new("./input/sample_input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut hands = vec!();
    BufReader::new(file).lines()
    .for_each(|lr| match lr {
        Ok(l) => {
            let mut splitted_hand = l.split_whitespace();
            let hand = Hand {
                cards: splitted_hand.next().unwrap().to_string(),
                bid: splitted_hand.next().unwrap().parse().unwrap(),
            };
            hands.push(hand);
        },
        Err(_) => panic!("could not read line"),
    });

    let mut hand_iter = hands.iter();
    let hand1 = hand_iter.next().unwrap();
    let hand2 = hand_iter.next().unwrap();
    let stronger = hand1.is_stronger(hand2);
    println!("Hand1 stronger: {}", stronger);
    let total_winnings = 0;
    Ok(total_winnings)
}
