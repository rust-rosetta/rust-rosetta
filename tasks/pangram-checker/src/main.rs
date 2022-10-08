use pangram_checker::{is_pangram_via_bitmask, is_pangram_via_hashset, is_pangram_via_sort};

fn main() {
    let examples = [
        "The quick brown fox jumps over the lazy dog",
        "The quick white cat jumps over the lazy dog",
    ];

    for &text in &examples {
        let is_pangram_sort = is_pangram_via_sort(text);
        println!("Is \"{}\" a pangram via sort? - {}", text, is_pangram_sort);

        let is_pangram_bitmask = is_pangram_via_bitmask(text);
        println!(
            "Is \"{}\" a pangram via bitmask? - {}",
            text, is_pangram_bitmask
        );

        let is_pangram_hashset = is_pangram_via_hashset(text);
        println!(
            "Is \"{}\" a pangram via bitmask? - {}",
            text, is_pangram_hashset
        );
    }
}
