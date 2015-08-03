extern crate rand;
extern crate mio;

mod game_server;

mod game {
    //! 'game' module is used for handling the data and logic for a card game. 
    //! 'server' module will issue the creation, and control of the various
    //! game components.

    #![allow(dead_code)]
    
    /* Static Array's corresponding to the ordinal values and suites of a card */
    static SUITES:[&'static str;4] = ["Heart", "Spade", "Diamond", "Club"];
    static VALUES:[&'static str;13] = ["Ace","Two", "Three", "Four", "Five",
                                       "Six", "Seven", "Eight", "Nine","Ten",
                                       "Jack", "Queen", "King"];
    use rand::{thread_rng,Rng};    
    use std::ops::Index;

    /// Struct that represents a Playing Card belonging to a Deck or Hand
    /// Can be copied and moved, size is known at runtime.
    #[derive(Copy,Clone)]
    pub struct Card {
        value:usize,
        suite:usize,
    }
    
    impl Card {        
        /// Generates a new Card.
        pub fn new(_val:usize, _suite:usize)->Card {
            Card{value:_val, suite:_suite}
        }

        /// Returns a pair of type &'static str 
        /// representing (value, suite)
        pub fn to_string_pair(&self)->(&'static str,&'static str) {
            (VALUES[self.value], SUITES[self.suite])
        }
    }

    /// A Container for Cards including a draw pile and a discard pile.
    pub struct Deck {
        card_deck:Vec<Card>,
        discard_pile:Vec<Card>,
        current:usize,
    }
    
    impl Deck {

        ///Creates a new, unshuffled deck containing the standard 52 cards.
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

        /// Merges the Decks discard pile with the draw pile and shuffles
        /// the draw pile in place. 
        pub fn shuffle(&mut self) {
            let mut rng = thread_rng();
            while self.discard_pile.len() > 0 {
                let card = self.discard_pile.pop();
                self.card_deck.push(card.unwrap());
            }
            rng.shuffle(&mut self.card_deck);
        }
    }

    impl Iterator for Deck {
        type Item = Card;

        /// impl of Iterator::next function allows use of foreach style
        /// iterator on a deck type directly.
        /// # Examples
        /// '''
        /// let deck = Deck::new();
        /// for card in deck {
        ///     let c_tup = card.to_string_pair();
        ///     println!("{} of {}s",c_tup.0,c_tup.1);
        /// }
        /// '''
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

        /// impl of Index::index allows indexed access to deck
        /// # Examples
        /// '''
        /// let deck = Deck::new();
        /// let card = deck[1];
        /// let c_tup = card.to_string_pair();
        /// println!("A {} of {}s is a Two of Hearts", c_tup.0, c_tup.1);
        /// '''
        fn index<'a>(&'a self, _index:usize)->&'a Card {
            &self.card_deck[_index]
        }
    }
}

#[test]
fn test_1() {
    #![allow(unused_variables)]
    let deck = game::Deck::new();
    let card = deck[0];
    let tup = card.to_string_pair();
    assert!(tup.0 == "Ace" && tup.1 == "Heart");
}

#[test]
fn test_2() {
    #![allow(unused_variables)]
    let deck = game::Deck::new();
    let mut count = 0;
    for card in deck {
        count += 1;
    }
    assert!(count == 52);
}


fn main() {
    #![allow(dead_code)]
    let mut deck = game::Deck::new();
    deck.shuffle();
    for card in deck {
        let tup = card.to_string_pair();
        println!("{} of {}s", tup.0, tup.1);
    }
}
