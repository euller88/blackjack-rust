use super::deck::*;
use std::cmp;
use std::fmt;

#[derive(PartialEq, Eq)]
pub enum State {
    PLAYER,
    DEALER,
    HANDOVER,
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl fmt::Display for Hand {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut strs: Vec<String> = Vec::new();
        for card in self.cards.iter() {
            strs.push(card.to_string());
        }
        write!(fmt, "{}", strs.join(","))
    }
}

impl Hand {
    pub fn dealer_hand(&self) -> String {
        return self.cards[0].to_string() + ", **HIDDEN**";
    }
    pub fn min_score(&self) -> u8 {
        let mut score: u8 = 0;
        for card in self.cards.iter() {
            score += cmp::min(card.rank as u8, 10 as u8);
        }
        return score;
    }
    pub fn score(&self) -> u8 {
        let score: u8 = self.min_score();

        if score > 11 {
            return score;
        }

        for card in self.cards.iter() {
            if card.rank == Rank::ACE {
                return score + 10;
            }
        }

        return score;
    }
}

fn draw(cards: Vec<Card>) -> (Card, Vec<Card>) {
    let v = cards.clone();

    return (v[0], v[1..].to_vec());
}

pub struct Game {
    deck: Vec<Card>,
    pub state: State,
    pub player: Hand,
    pub dealer: Hand,
}

impl Game {
    pub fn new() -> Game {
        Game {
            dealer: Hand { cards: Vec::new() },
            player: Hand { cards: Vec::new() },
            deck: Vec::new(),
            state: State::PLAYER,
        }
    }
    pub fn shuffle(&mut self) {
        self.deck.splice(.., cards(&[&(deck(3)), &(shuffle)]));
    }
    pub fn deal(&mut self) {
        for _ in 0..2 {
            let (card, aux) = draw(self.deck.to_vec());
            self.player.cards.push(card);
            self.deck.splice(.., aux.to_vec());
        }
        for _ in 0..2 {
            let (card, aux) = draw(self.deck.to_vec());
            self.dealer.cards.push(card);
            self.deck.splice(.., aux.to_vec());
        }
        self.state = State::PLAYER;
    }
    pub fn stand(&mut self) {
        match self.state {
            State::PLAYER => self.state = State::DEALER,
            State::DEALER => self.state = State::HANDOVER,
            _ => panic!("Trying to stand in an invalid state"),
        }
    }
    pub fn hit(&mut self) {
        let (card, aux) = draw(self.deck.to_vec());
        match self.state {
            State::PLAYER => {
                self.player.cards.push(card);
                self.deck.splice(.., aux);
                if self.player.score() >= 21 {
                    self.stand();
                }
            }
            State::DEALER => {
                self.dealer.cards.push(card);
                self.deck.splice(.., aux);
                if self.dealer.score() >= 21 {
                    self.stand();
                }
            }
            _ => panic!("It isn't currently any player's turn"),
        }
    }
    pub fn end(&mut self) {
        let p_score = self.player.score();
        let d_score = self.dealer.score();

        println!("==FINAL HANDS==");
        println!("Player: {}", self.player);
        println!("Score: {}", p_score);
        println!("Dealer: {}", self.dealer);
        println!("Score: {}", d_score);

        if p_score > 21 {
            println!("You busted");
        } else if d_score > 21 {
            println!("Dealer busted");
        } else if p_score < d_score {
            println!("You lose");
        } else if d_score < p_score {
            println!("You win");
        } else if d_score == p_score {
            println!("Draw");
        }
        println!("");

        self.player.cards.clear();
        self.dealer.cards.clear();
    }
}
