use std::collections::HashSet;

use itertools::{repeat_n, Itertools};

use krypto_checker::compute_impossibles;

const MAX_CARD_NUMBER: i64 = 10;
const CARD_DUPLICATES: usize = 3;
const NUM_HAND_CARDS: usize = 5;

fn main() {
    let entire = repeat_n(1..=MAX_CARD_NUMBER, CARD_DUPLICATES)
        .flatten()
        .sorted()
        .collect_vec();
    let combs = entire.iter().copied().combinations(NUM_HAND_CARDS).unique();
    let within: HashSet<_> = entire.iter().copied().collect();

    for hand in combs {
        let impossibles = compute_impossibles(&hand, within.clone());
        if !impossibles.is_empty() {
            println!("{hand:?} != {impossibles:?}");
        }
    }
}
