mod deck;
mod game;

#[cfg(test)]
mod tests {
    use super::deck::*;

    #[test]
    fn test_string() {
        let c = Card::new(Suit::SPADE, Rank::ACE);
        assert_eq!(c.to_string(), "Ace of Spades")
    }

    #[test]
    fn test_cards() {
        let v = cards(&[]);
        assert!(v.len() == 13 * 4)
    }

    #[test]
    fn test_default_sort() {
        let v = cards(&[&(default_sort)]);
        let exp = Card::new(Suit::SPADE, Rank::ACE);
        assert_eq!(exp, v[0])
    }

    #[test]
    fn test_filter() {
        let f1 = |c: &Card| -> bool {
            return c.rank == Rank::TWO || c.rank == Rank::THREE;
        };
        let f2 = filter(f1);
        let v = cards(&[&(shuffle), &f2]);
        assert!(!v
            .iter()
            .any(|c| c.rank == Rank::TWO || c.rank == Rank::THREE))
    }

    #[test]
    fn test_deck() {
        let v = cards(&[&deck(3)]);
        assert!(v.len() == 13 * 4 * 3)
    }
}

fn main() {
    let mut g = game::Game::new();
    g.shuffle();

    for _ in 0..10 {
        g.deal();

        let mut input: String = String::new();

        while g.state == game::State::PLAYER {
            println!("Player: {} ({})", g.player, g.player.score());
            println!("Dealer: {}", g.dealer.dealer_hand());
            println!("What will you do? (h)it, (s)tand");

            std::io::stdin().read_line(&mut input).unwrap();

            match input.as_str().trim() {
                "h" => g.hit(),
                "s" => g.stand(),
                _ => println!("invalid option: {}", input),
            }

            input = "".to_string();
        }

        while g.state == game::State::DEALER {
            if g.dealer.score() <= 16 || (g.dealer.score() == 17 && g.dealer.min_score() != 17) {
                g.hit();
            } else {
                g.stand();
            }
        }

        g.end();
    }
}
