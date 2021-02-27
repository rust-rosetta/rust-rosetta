// extern crate itertools;
use itertools::Itertools;

fn get_kolakoski_sequence(iseq: &[usize], size: &usize) -> Vec<usize> {
    assert!(*size > 0);
    assert!(!iseq.is_empty());

    let mut kseq: Vec<usize> = Vec::default();

    // create an itertor which keeps repeating the initial sequence infinitely
    let repeater = iseq.iter().cloned().cycle();

    // push the very first element, repeated as many times as the number
    kseq.extend_from_slice(&vec![*iseq.get(0).unwrap()].repeat(*iseq.get(0).unwrap()));

    //start cycling throught the initial sequence, but skip the very first one
    for (k_counter, elem) in repeater.enumerate().skip(1) {
        // push the given element
        kseq.push(elem);

        // and repeat the current element as many times
        // as it's needed based on the previous elements
        kseq.extend_from_slice(&vec![elem].repeat(*kseq.get(k_counter).unwrap() - 1));

        // finish generation when the Kolakoski sequence has reached the given length
        if kseq.len() >= *size {
            break;
        }
    }

    // truncate it as it might have more elements than needed
    kseq[0..*size].to_vec()
}

fn is_kolakoski(kseq: &[usize]) -> bool {
    assert!(!kseq.is_empty());

    // calculate the RLE
    let rle: Vec<usize> = kseq
        .iter()
        .batching(|it| {
            it.next()
                .map(|v| it.take_while_ref(|&v2| v2 == v).count() + 1)
        })
        .collect();

    rle.iter().zip(kseq).filter(|&(a, b)| a == b).count() == rle.len()
}

fn main() {
    let lengths = vec![20, 20, 30, 30];
    let seqs = vec![vec![1, 2], vec![2, 1], vec![1, 3, 1, 2], vec![1, 3, 2, 1]];

    for (seq, length) in seqs.iter().zip(&lengths) {
        let kseq = get_kolakoski_sequence(&seq, length);

        println!("Starting sequence: {:?}", seq);
        println!("Kolakoski sequence: {:?}", kseq);
        println!("Possible Kolakoski sequence? {:?}", is_kolakoski(&kseq));
    }
}

#[cfg(test)]
mod tests {
    use super::{get_kolakoski_sequence, is_kolakoski};

    #[test]
    fn test_get_kolakoski_sequence() {
        let input = vec![1, 2];
        let output = get_kolakoski_sequence(&input, &10);
        assert_eq!(output, vec![1, 2, 2, 1, 1, 2, 1, 2, 2, 1]);

        let input = vec![2, 1];
        let output = get_kolakoski_sequence(&input, &10);
        assert_eq!(output, vec![2, 2, 1, 1, 2, 1, 2, 2, 1, 2]);

        let input = vec![1, 3, 2, 1];
        let output = get_kolakoski_sequence(&input, &10);
        assert_eq!(output, vec![1, 3, 3, 3, 2, 2, 2, 1, 1, 1]);
    }

    #[test]
    fn test_is_kolakoski() {
        let input = vec![1, 2, 2, 1, 1, 2, 1, 2, 2, 1];
        assert_eq!(is_kolakoski(&input), true);

        let input = vec![2, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 2, 2, 1, 2];
        assert_eq!(is_kolakoski(&input), true);

        let input = vec![1, 3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 3, 3, 2, 2, 1, 1, 3, 2];
        assert_eq!(is_kolakoski(&input), false);
    }
}
