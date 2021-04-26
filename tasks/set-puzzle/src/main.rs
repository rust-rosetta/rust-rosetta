use itertools::Itertools;
use rand::Rng;

const DECK_SIZE: usize = 81;
const NUM_ATTRIBUTES: usize = 4;
const ATTRIBUTES: [&[&str]; NUM_ATTRIBUTES] = [
    &["red", "green", "purple"],
    &["one", "two", "three"],
    &["oval", "squiggle", "diamond"],
    &["solid", "open", "striped"],
];

fn get_random_card_indexes(num_of_cards: usize) -> Vec<usize> {
    let mut selected_cards: Vec<usize> = Vec::with_capacity(num_of_cards);
    let mut rng = rand::thread_rng();
    loop {
        let idx = rng.gen_range(0..DECK_SIZE);
        if !selected_cards.contains(&idx) {
            selected_cards.push(idx);
        }
        if selected_cards.len() == num_of_cards {
            break;
        }
    }

    selected_cards
}

fn run_game(num_of_cards: usize, minimum_number_of_sets: usize) {
    println!(
        "\nGAME: # of cards: {} # of sets: {}",
        num_of_cards, minimum_number_of_sets
    );

    // generate the deck with 81 unique cards
    let deck = (0..NUM_ATTRIBUTES)
        .map(|_| (0..=2_usize))
        .multi_cartesian_product()
        .collect::<Vec<_>>();

    // closure to return true if the three attributes are the same, or each of them is different
    let valid_attribute =
        |a: usize, b: usize, c: usize| -> bool { a == b && b == c || (a != b && b != c && a != c) };

    // closure to test all attributes, each of them should be true to have a valid set
    let valid_set = |t: &Vec<&Vec<usize>>| -> bool {
        for attr in 0..NUM_ATTRIBUTES {
            if !valid_attribute(t[0][attr], t[1][attr], t[2][attr]) {
                return false;
            }
        }
        true
    };

    loop {
        // select the required # of cards from the deck randomly
        let selected_cards = get_random_card_indexes(num_of_cards)
            .iter()
            .map(|idx| deck[*idx].clone())
            .collect::<Vec<_>>();

        // generate all combinations, and filter/keep only which are valid sets
        let valid_sets = selected_cards
            .iter()
            .combinations(3)
            .filter(|triplet| valid_set(triplet))
            .collect::<Vec<_>>();

        // if the # of the sets is matching the requirement, print it and finish
        if valid_sets.len() == minimum_number_of_sets {
            print!("SELECTED CARDS:");
            for card in &selected_cards {
                print!("\ncard: ");
                for attr in 0..NUM_ATTRIBUTES {
                    print!("{}, ", ATTRIBUTES[attr][card[attr]]);
                }
            }

            print!("\nSets:");
            for triplet in &valid_sets {
                print!("\nSet: ");
                for card in triplet {
                    for attr in 0..NUM_ATTRIBUTES {
                        print!("{}, ", ATTRIBUTES[attr][card[attr]]);
                    }
                    print!(" | ");
                }
            }

            break;
        }

        //otherwise generate again
    }
}
fn main() {
    run_game(9, 4);
    run_game(12, 6);
}
