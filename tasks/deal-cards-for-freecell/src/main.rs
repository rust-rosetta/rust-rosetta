// Code available at https://rosettacode.org/wiki/Linear_congruential_generator#Rust
extern crate linear_congruential_generator;

use linear_congruential_generator::{MsLcg, Rng, SeedableRng};

// We can't use `rand::Rng::shuffle` because it uses the more uniform `rand::Rng::gen_range`
// (`% range` is subject to modulo bias).  If an exact match of the old dealer is not needed,
// `rand::Rng::shuffle` should be used.
fn shuffle<T>(rng: &mut MsLcg, deck: &mut [T]) {
    let len = deck.len() as u32;
    for i in (1..len).rev() {
        let j = rng.next_u32() % (i + 1);
        deck.swap(i as usize, j as usize);
    }
}

fn gen_deck() -> Vec<String> {
    const RANKS: [char; 13] = ['A','2','3','4','5','6','7','8','9','T','J','Q','K'];
    const SUITS: [char; 4] = ['C', 'D', 'H', 'S'];

    let render_card = |card: usize| {
        let (suit, rank) = (card % 4, card / 4);
        format!("{}{}", RANKS[rank], SUITS[suit])
    };

    (0..52).map(render_card).collect()
}

fn deal_ms_fc_board(seed: u32) -> Vec<String> {
    let mut rng = MsLcg::from_seed(seed);
    let mut deck = gen_deck();

    shuffle(&mut rng, &mut deck);
    deck.reverse();

    deck.chunks(8).map(|row| row.join(" ")).collect::<Vec<_>>()
}

fn main() {
    let seed = std::env::args()
        .nth(1)
        .and_then(|n| n.parse().ok())
        .expect("A 32-bit seed is required");

    for row in deal_ms_fc_board(seed) {
        println!(": {}", row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_one() {
        assert_eq!(
            deal_ms_fc_board(1),
            [
                "JD 2D 9H JC 5D 7H 7C 5H",
                "KD KC 9S 5S AD QC KH 3H",
                "2S KS 9D QD JS AS AH 3C",
                "4C 5C TS QH 4H AC 4D 7S",
                "3S TD 4S TH 8H 2C JH 7D",
                "6D 8S 8D QS 6C 3D 8C TC",
                "6S 9C 2H 6H"
            ]
        );
    }

    #[test]
    fn seed_617() {
        assert_eq!(
            deal_ms_fc_board(617),
            [
                "7D AD 5C 3S 5S 8C 2D AH",
                "TD 7S QD AC 6D 8H AS KH",
                "TH QC 3H 9D 6S 8D 3D TC",
                "KD 5H 9S 3C 8S 7H 4D JS",
                "4C QS 9C 9H 7C 6H 2C 2S",
                "4S TS 2H 5D JC 6C JH QH",
                "JD KS KC 4H"
            ]
        );
    }
}
