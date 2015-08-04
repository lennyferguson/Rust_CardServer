use game::*;

#[test]
/// Test the index fn
fn first_last_test() {
    #![allow(unused_variables)]
    let deck = Deck::new();
    let first = deck[0];
    let last = deck[51];
    let f_tup = first.to_string_pair();
    let l_tup = last.to_string_pair();
    assert!(f_tup.0 == "Ace" && f_tup.1 == "Diamond");
    assert!(l_tup.0 == "King" && l_tup.1 == "Spade");
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

#[test]
fn overdraw_test() {
    #![allow(unused_variables)]
    let mut deck = Deck::new();
    let mut v:Vec<Hand> = Vec::new();
    for x in 0..11 {
        let mut hand = Hand::new();
        draw_to_hand(&mut deck, &mut hand, 5);
        v.push(hand);
    }
    assert!(deck.count() == 0);
    assert!(v[10].count() == 2);
}
