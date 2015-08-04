use game::*;

#[test]
/// Test the index fn
fn index_test() {
    #![allow(unused_variables)]
    let deck = Deck::new();
    let card = deck[0];
    let tup = card.to_string_pair();
    assert!(tup.0 == "Ace" && tup.1 == "Heart");
}

#[test]
/// Test the iterator
fn iterator_test() {
    #![allow(unused_variables)]
    let deck = Deck::new();
    let mut count = 0;
    for card in deck {
        count += 1;
    }
    assert!(count == 52);
}

#[test]
/// Test the draw_to_hand fn
fn draw_to_hand_test() {
    #![allow(unused_variables)]
    let mut deck = Deck::new();
    let mut hand = Hand::new();
    draw_to_hand(&mut deck,&mut hand,5);
    assert!(deck.count() == (47));
    assert!(hand.count() == 5);
}

#[test]
/// Test the play_to_discard fn
fn play_to_discard_test() {
    #![allow(unused_variables)]
    let mut deck = Deck::new();
    let mut hand = Hand::new();
    draw_to_hand(&mut deck,&mut hand,5);
    play_to_discard(&mut deck, &mut hand);
    assert!(hand.count() == 0);
    deck.shuffle();
    assert!(deck.count() == 52);
}
