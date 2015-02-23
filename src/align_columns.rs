// Implements http://rosettacode.org/wiki/Align_columns
const TEST_STR: &'static str =
    "Given$a$text$file$of$many$lines,$where$fields$within$a$line$\nare$delineated\
    $by$a$single$'dollar'$character,$write$a$program\nthat$aligns$each$column$of\
    $fields$by$ensuring$that$words$in$each$\ncolumn$are$separated$by$at$least$one\
    $space.\nFurther,$allow$for$each$word$in$a$column$to$be$either$left$\n\
    justified,$right$justified,$or$center$justified$within$its$column.\n";

#[cfg(not(test))]
fn main() {
    let (chunks, max_lengths) = align_columns(TEST_STR);
    print_aligned_columns(&chunks, &max_lengths);
}

fn align_columns(text: &str) -> (Vec<Vec<String>>, Vec<usize>) {
    let lines: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();
    let mut max_lengths: Vec<usize> = Vec::new();
    let mut chunks: Vec<Vec<String>> = Vec::new();

    for i in (0..lines.len()) {
        let ref input = lines[i];
        let split_input: Vec<String> = input[..].split('$').map(|s| s.to_string()).collect();
        chunks.push(split_input.clone());
        let v: Vec<usize> = split_input.iter().map(|chunk| chunk.len() ).collect();

        for i in (0..v.len()) {
            if i < max_lengths.len() {
                max_lengths[i] = std::cmp::max(max_lengths[i], v[i]);
            } else {
                max_lengths.push(v[i]);
            }
        }
    }

    (chunks, max_lengths)
}

fn print_aligned_columns(chunks: &Vec<Vec<String>>, max_lengths: &Vec<usize>) {
    // left aligned
    for i in (0..chunks.len()) {
        for j in (0..chunks[i].len()) {
            print!("{0:<1$}", chunks[i][j], 1 + max_lengths[j]);
        }
        println!("");
    }
    println!("");
    // right aligned
    for i in (0..chunks.len()) {
        for j in (0..chunks[i].len()) {
            print!("{0:>1$}", chunks[i][j], 1 + max_lengths[j]);
        }
        println!("");
    }
    println!("");
    // center aligned
    for i in (0..chunks.len()) {
        for j in (0..chunks[i].len()) {
            let ref string: String = chunks[i][j];
            let spaces: usize = 1 + max_lengths[j] - string.len();
            for _ in (0..spaces>>1) {
                print!(" ");
            }
            print!("{}", string);
            for _ in (0..spaces - (spaces>>1)) {
                print!(" ");
            }
        }
        println!("");
    }
}

#[test]
fn test_result() {
    let (chunks, max_lengths) = align_columns(TEST_STR);
    for chunkset in &chunks {
        // the number of words in a chunkset is <= the number of values in max_lengths
        assert!(chunkset.len() <= max_lengths.len());
        for j in (0..chunkset.len()) {
            // a word in a chunkset cannot be longer than max_lengths
            assert!(chunkset[j].len() <= max_lengths[j]);
        }
    }
    print_aligned_columns(&chunks, &max_lengths);
}
