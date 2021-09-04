use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};
use advtools::prelude::*;
use advtools::input::{iter_lines, to_u8};

type Deck = ArrayVec<[u8; 50]>;

fn play(mut deck1: Deck, mut deck2: Deck, rec: bool) -> (bool, Deck) {
    let mut seen = HashSet::new();
    loop {
        let mut hasher = DefaultHasher::new();
        deck1.hash(&mut hasher);
        deck2.hash(&mut hasher);
        if !seen.insert(hasher.finish()) {
            return (true, deck1);
        } else if deck1.is_empty() {
            return (false, deck2);
        } else if deck2.is_empty() {
            return (true, deck1);
        }

        let (card1, card2) = (deck1.remove(0), deck2.remove(0));

        let player1_won = if rec && card1 <= deck1.len() as u8 && card2 <= deck2.len() as u8 {
            play(deck1.iter().cloned().take(card1 as usize).collect(),
                 deck2.iter().cloned().take(card2 as usize).collect(), true).0
        } else {
            card1 > card2
        };

        if player1_won {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    }
}

fn score(deck: &Deck) -> usize {
    deck.iter().rev().enumerate().map(|(i, &n)| n as usize * (i+1)).sum()
}

fn main() {
    let mut deck1 = Deck::new();
    let mut deck2 = Deck::new();
    let mut deck = &mut deck1;
    for line in iter_lines().skip(1) {
        if line.starts_with("Player 2") {
            deck = &mut deck2;
        } else {
            deck.push(to_u8(line));
        }
    }

    advtools::verify("Normal game", score(&play(deck1.clone(), deck2.clone(), false).1), 32448);
    advtools::verify("Recursive game", score(&play(deck1, deck2, true).1), 32949);
}
