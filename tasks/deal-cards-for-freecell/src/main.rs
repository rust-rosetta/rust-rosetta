//! Based on JavaScript.

struct MSVCRandGen {
    seed: u32,
}

impl MSVCRandGen {
    fn rand(&mut self) -> u32 {
        self.seed = (self.seed.wrapping_mul(214013).wrapping_add(2531011)) % 0x80000000;
        assert!(self.seed >> 16 < 32768);
        (self.seed >> 16) & 0x7FFF
    }
    fn max_rand(&mut self, mymax: u32) -> u32 {
        self.rand() % mymax
    }
    fn shuffle<T>(&mut self, deck: &mut [T]) {
        if deck.len() > 0 {
            let mut i = (deck.len() as u32) - 1;
            while i > 0 {
                let j = self.max_rand(i + 1);
                deck.swap(i as usize, j as usize);
                i -= 1;
            }
        }
    }
}

fn deal_ms_fc_board(seed: u32) -> String {
    let mut randomizer = MSVCRandGen { seed: seed };
    let num_cols = 8;

    let mut columns = vec![Vec::new(); num_cols];
    let mut deck: Vec<_> = (0..4 * 13).collect();

    let rank_strings: Vec<char> = "A23456789TJQK".chars().collect();
    let suit_strings: Vec<char> = "CDHS".chars().collect();

    randomizer.shuffle(&mut deck);

    deck.reverse();

    for (i, card) in deck.iter().enumerate() {
        columns[i % num_cols].push(*card);
    }

    let render_card = |card: usize| -> String {
        let (suit, rank) = (card % 4, card / 4);
        format!("{}{}", rank_strings[rank], suit_strings[suit])
    };

    let render_column = |col: Vec<usize>| -> String {
        format!(": {}\n",
                col.into_iter().map(&render_card).collect::<Vec<String>>().join(" "))
    };

    columns.into_iter().map(render_column).collect::<Vec<_>>().join("")
}

fn main() {
    let arg: u32 = std::env::args().nth(1).and_then(|n| n.parse().ok()).expect("I need a number.");
    print!("{}", deal_ms_fc_board(arg));
}
