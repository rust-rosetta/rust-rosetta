use std::collections::HashMap;

const DNA: &str = "CGTAAAAAATTACAACGTCCTTTGGCTATCTCTTAAACTCCTGCTAAATGCTCGTGCTTTCCAATTATGTAAGCGTTCCGAGACGGGGTGGTCGATTCTGAGGACAAAGGTCAAGATGGAGCGCATCGAACGCAATAAGGATCATTTGATGGGACGTTTCGTCGACAAAGTCTTGTTTCGAGAGTAACGGCTACCGTCTTCGATTCTGCTTATAACACTATGTTCTTATGAAATGGATGTTCTGAGTTGGTCAGTCCCAATGTGCGGGGTTTCTTTTAGTACGTCGGGAGTGGTATTATATTTAATTTTTCTATATAGCGATCTGTATTTAAGCAATTCATTTAGGTTATCGCCGCGATGCTCGGTTCGGACCGCCAAGCATCTGGCTCCACTGCTAGTGTCCTAAATTTGAATGGCAAACACAAATAAGATTTAGCAATTCGTGTAGACGACCGGGGACTTGCATGATGGGAGCAGCTTTGTTAAACTACGAACGTAAT";
const SEQUENCE_LIMIT: u32 = 50;

fn main() {
    let mut base_count = HashMap::new();
    let mut total_count = 0;

    print!("Sequence:");

    for base in DNA.chars() {
        if total_count % SEQUENCE_LIMIT == 0 {
            print!("\n{:3}: ", total_count);
        }
        print!("{}", base);
        total_count += 1;
        let count = base_count.entry(base).or_insert(0); // Return current count for base or insert 0
        *count += 1;
    }

    println!("\n");
    println!("Base count:");
    println!("-----------");

    let mut base_count: Vec<_> = base_count.iter().collect(); // HashMaps can't be sorted, so collect into Vec
    base_count.sort_by_key(|bc| bc.0); // Sort bases alphabetically

    for (base, count) in base_count.iter() {
        println!("{}: {:3}", base, count);
    }

    println!("\nTotal: {}", total_count);
}
