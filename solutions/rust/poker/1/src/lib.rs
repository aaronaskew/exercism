mod cards;

use cards::*;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        return hands.to_vec();
    }

    let mut hands: Vec<Hand> = hands.iter().map(|hand| Hand::new(hand)).collect();

    hands.sort();
    hands.reverse();

    let winning_hand = &hands[0];

    hands
        .iter()
        .filter_map(|hand| {
            if hand.rank == winning_hand.rank {
                Some(hand.orig)
            } else {
                None
            }
        })
        .collect()
}
