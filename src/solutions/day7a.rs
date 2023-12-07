use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use crate::file_reader;
use crate::solutions::day7a::HandType::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};

#[derive(Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Clone)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,

    NotSet = 0
}

#[derive(Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Clone)]
enum CardType {CardA = 12, CardK = 11, CardQ = 10, CardJ = 9, CardT = 8, Card9 = 7, Card8 = 6, Card7 = 5, Card6 = 4, Card5 = 3, Card4 = 2, Card3 = 1, Card2 = 0, None = -1}

#[derive(Debug, Eq, Clone)]
struct Hand {
    hand: Vec<CardType>,
    bid: i64,
    hand_type: HandType,
}

impl Hand {
    pub fn new<'a>() -> Hand {
        Hand {
            hand: vec![],
            bid: -1,
            hand_type: HandType::NotSet
        }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.hand_type == other.hand_type && self.hand == other.hand;
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type.cmp(&other.hand_type) == Equal {
            return self.hand.cmp(&other.hand);
        }
        return self.hand_type.cmp(&other.hand_type);
    }
}

pub fn solve_a() {
    let lines = file_reader::read_file_lines("input_data/day7/day7.in");
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let split_line = line.split(" ");
        let split_vec = split_line.collect::<Vec<&str>>();

        let hand = split_vec[0];
        let bid = split_vec[1].parse::<i64>().unwrap();

        let mut current_hand: Hand = Hand::new();

        let mut count_cards: HashMap<char, i32> = HashMap::new();
        hand.chars().for_each(|c| {count_cards.insert(c, count_cards.get(&c).unwrap_or(&0) + 1);});
        current_hand.hand = hand.chars().map(
            |c| match c {
                'A' => CardType::CardA,
                'K' => CardType::CardK,
                'Q' => CardType::CardQ,
                'J' => CardType::CardJ,
                'T' => CardType::CardT,
                '9' => CardType::Card9,
                '8' => CardType::Card8,
                '7' => CardType::Card7,
                '6' => CardType::Card6,
                '5' => CardType::Card5,
                '4' => CardType::Card4,
                '3' => CardType::Card3,
                '2' => CardType::Card2,
                _   => CardType::None
            }
        ).collect();

        current_hand.bid = bid;

        current_hand.hand_type = match count_cards.len() {
            5 => HighCard,
            4 => OnePair,
            3 => {
                let cnt1 = *count_cards.iter().nth(0).unwrap().1;
                let cnt2 = *count_cards.iter().nth(1).unwrap().1;
                if (cnt1 == 1 && cnt2 == 1) || (cnt1 == 1 && cnt2 == 3) || (cnt1 == 3 && cnt2 == 1) {
                    ThreeOfKind
                } else {
                    TwoPair
                }
            },
            2 => {
                let cnt = *count_cards.iter().nth(0).unwrap().1;
                if cnt == 4 || cnt == 1 {
                    FourOfKind
                } else {
                    FullHouse
                }
            },
            1 => FiveOfKind,
            _ => panic!("Illegal length of hand!")
        };

        hands.push(current_hand.clone());
    }

    hands.sort();

    let mut total_bid = 0;

    for (i, hand) in hands.iter().enumerate() {
        total_bid += hand.bid*(i+1) as i64;
    }

    println!("{}", total_bid);
}