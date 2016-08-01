const TEST_STR: &'static str = r"Given$a$text$file$of$many$lines,$where$fields$within$a$line$
are$delineated$by$a$single$'dollar'$character,$write$a$program
that$aligns$each$column$of$fields$by$ensuring$that$words$in$each$
column$are$separated$by$at$least$one$space.
Further,$allow$for$each$word$in$a$column$to$be$either$left$
justified,$right$justified,$or$center$justified$within$its$column.
";

fn main() {
    let (chunks, max_lengths) = align_columns(TEST_STR);
    print_aligned_columns(&chunks, &max_lengths);
}

fn align_columns(text: &str) -> (Vec<Vec<String>>, Vec<usize>) {
    let lines: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();
    let mut max_lengths: Vec<usize> = Vec::new();
    let mut chunks: Vec<Vec<String>> = Vec::new();

    for line in &lines {
        let split_line: Vec<String> = line.split('$').map(|s| s.to_string()).collect();
        chunks.push(split_line.clone());
        let v: Vec<usize> = split_line.iter().map(|chunk| chunk.len()).collect();

        for i in 0..v.len() {
            if i < max_lengths.len() {
                max_lengths[i] = std::cmp::max(max_lengths[i], v[i]);
            } else {
                max_lengths.push(v[i]);
            }
        }
    }

    (chunks, max_lengths)
}

fn print_aligned_columns(chunks: &[Vec<String>], max_lengths: &[usize]) {
    // left aligned
    for chunk in chunks {
        for (i, split) in chunk.iter().enumerate() {
            print!("{0:<1$}", split, 1 + max_lengths[i]);
        }
        println!("");
    }
    println!("");
    // right aligned
    for chunk in chunks {
        for (i, split) in chunk.iter().enumerate() {
            print!("{0:>1$}", split, 1 + max_lengths[i]);
        }
        println!("");
    }
    println!("");
    // center aligned
    for chunk in chunks {
        for (i, split) in chunk.iter().enumerate() {
            let spaces: usize = 1 + max_lengths[i] - split.len();
            for _ in 0..spaces >> 1 {
                print!(" ");
            }
            print!("{}", split);
            for _ in 0..(spaces - (spaces >> 1)) {
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
        for j in 0..chunkset.len() {
            // a word in a chunkset cannot be longer than max_lengths
            assert!(chunkset[j].len() <= max_lengths[j]);
        }
    }
    print_aligned_columns(&chunks, &max_lengths);
}
