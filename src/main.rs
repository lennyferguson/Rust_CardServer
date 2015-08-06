extern crate rand;
extern crate mio;

mod game_server;
mod game;

/*Conditional compilation and execution of tests module when executing
  with the test directive. */
#[cfg(test)]
mod tests;

fn main() {
    #![allow(dead_code)]
    let mut deck = game::Deck::new();
    deck.shuffle();
    deck.sort();
    for card in deck {
        let tup = card.to_string_pair();
        println!("{} of {}s", tup.0, tup.1);
    }
}
