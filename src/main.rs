extern crate rand;
mod game {
    static SUITES:[&'static str;4] = ["Heart", "Spade", "Diamond", "Club"];
    static VALUES:[&'static str;13] = ["Ace","Two", "Three", "Four", "Five",
                                       "Six", "Seven", "Eight", "Nine","Ten",
                                       "Jack", "Queen", "King"];
    use rand::{thread_rng,Rng};    
    use std::ops::Index;
    #[derive(Copy,Clone)]
    pub struct Card {
        value:usize,
        suite:usize,
    }
    
    impl Card {
        pub fn new(_val:usize, _suite:usize)->Card {
            Card{value:_val, suite:_suite}
        }
        
        /* Returns a Tuple of type (String, String)
        represeting (Value,Suite) */
        pub fn to_string_pair(&self)->(String,String) {
            (VALUES[self.value].to_string(), SUITES[self.suite].to_string())
        }
    }
    
    pub struct Deck {
        card_deck:Vec<Card>,
        discard_pile:Vec<Card>,
        current:usize,
    }
    
    impl Deck {
        pub fn new()->Deck {
            let mut cards = Vec::new();
            let discard = Vec::new();
            for x in 0..4 {
                for y in 0..13 {
                    let card = Card::new(y,x);
                    cards.push(card);
                }
            }
            Deck{card_deck:cards, discard_pile:discard, current:0}
        }

        pub fn shuffle(&mut self) {
            let mut rng = thread_rng();
            for card in &self.discard_pile {
                self.card_deck.push(*card);
            }
            rng.shuffle(&mut self.card_deck);
        }
    }

    impl Iterator for Deck {
        type Item = Card;
        fn next(&mut self)->Option<Card> {
            let length = self.card_deck.len();
            if self.current < length {
                let index = self.current;
                self.current += 1;
                Some(self.card_deck[index])
            }
            else {
                self.current = 0;
                None
            }
        }
    }

    impl Index<usize> for Deck {
        type Output = Card;
        fn index<'a>(&'a self, _index:usize)->&'a Card {
            &self.card_deck[_index]
        }
    }
}

fn main() {
    let mut deck = game::Deck::new();
    deck.shuffle();
    for card in deck {
        let tup = card.to_string_pair();
        println!("{} of {}s", tup.0, tup.1)
    }
    /*
    let c = deck[0];
    let t = c.to_string_pair();
    println!("{} of {}s", t.0, t.1);
     */
}
