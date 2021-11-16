use rand::prelude::*;
use std::fmt;
use std::slice::Iter;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    SPADE,
    DIAMOND,
    CLUB,
    HEART,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rank {
    ACE = 1,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

impl fmt::Display for Suit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::SPADE => write!(fmt, "Spades"),
            Suit::DIAMOND => write!(fmt, "Diamonds"),
            Suit::CLUB => write!(fmt, "Clubs"),
            Suit::HEART => write!(fmt, "Hearts"),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::ACE => write!(fmt, "Ace"),
            Rank::TWO => write!(fmt, "Two"),
            Rank::THREE => write!(fmt, "Three"),
            Rank::FOUR => write!(fmt, "Four"),
            Rank::FIVE => write!(fmt, "Five"),
            Rank::SIX => write!(fmt, "Six"),
            Rank::SEVEN => write!(fmt, "Seven"),
            Rank::EIGHT => write!(fmt, "Eight"),
            Rank::NINE => write!(fmt, "Nine"),
            Rank::TEN => write!(fmt, "Ten"),
            Rank::JACK => write!(fmt, "Jack"),
            Rank::QUEEN => write!(fmt, "Queen"),
            Rank::KING => write!(fmt, "King"),
        }
    }
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Suit::SPADE, Suit::DIAMOND, Suit::CLUB, Suit::HEART];
        SUITS.iter()
    }
}

impl Rank {
    pub fn iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 13] = [
            Rank::ACE,
            Rank::TWO,
            Rank::THREE,
            Rank::FOUR,
            Rank::FIVE,
            Rank::SIX,
            Rank::SEVEN,
            Rank::EIGHT,
            Rank::NINE,
            Rank::TEN,
            Rank::JACK,
            Rank::QUEEN,
            Rank::KING,
        ];
        RANKS.iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} of {}", self.rank, self.suit)
    }
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            suit: suit,
            rank: rank,
        }
    }
}

#[allow(dead_code)]
fn abs_rank(card: &Card) -> u8 {
    return ((card.suit as u8) * 13) + card.rank as u8;
}

pub fn shuffle(cards: Vec<Card>) -> Vec<Card> {
    let mut v: Vec<Card> = cards.clone();
    v.shuffle(&mut thread_rng());
    return v;
}

#[allow(dead_code)]
pub fn default_sort(cards: Vec<Card>) -> Vec<Card> {
    let mut v: Vec<Card> = cards.clone();
    v.sort_by(|a: &Card, b: &Card| abs_rank(a).partial_cmp(&abs_rank(b)).unwrap());
    return v;
}

#[allow(dead_code)]
pub fn filter(filter_fn: fn(&Card) -> bool) -> impl Fn(Vec<Card>) -> Vec<Card> {
    return move |x: Vec<Card>| -> Vec<Card> {
        let mut v: Vec<Card> = x.clone();
        v.retain(|y: &Card| !filter_fn(y));
        return v;
    };
}

pub fn deck(n: u8) -> impl Fn(Vec<Card>) -> Vec<Card> {
    return move |x: Vec<Card>| -> Vec<Card> {
        let mut v: Vec<Card> = Vec::new();
        for _i in 0..n {
            v.append(&mut x.clone())
        }
        return v;
    };
}

pub fn cards(opts: &[&dyn Fn(Vec<Card>) -> Vec<Card>]) -> Vec<Card> {
    let mut v: Vec<Card> = Vec::new();
    for suit in Suit::iterator() {
        for rank in Rank::iterator() {
            v.push(Card::new(*suit, *rank));
        }
    }

    for opt in opts.iter() {
        let c = v.clone();
        v.splice(.., opt(c));
    }

    return v.clone();
}
