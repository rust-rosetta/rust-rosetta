#[derive(Debug, PartialEq)]
enum AliquotType {
    Terminating,
    Perfect,
    Amicable,
    Sociable,
    Aspiring,
    Cyclic,
    NonTerminating,
}

#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn classify_aliquot(num: i64) -> (AliquotType, Vec<i64>) {
    let limit = 1i64 << 47; // 140737488355328
    let mut terms = Some(num).into_iter().collect::<Vec<_>>();
    for i in 0..16 {
        let n = terms[i];
        let divsum = (1..(n + 1) / 2 + 1)
            .filter(|&x| n % x == 0 && n != x)
            .fold(0, |sum, x| sum + x);
        let classification = if divsum == 0 {
            Some(AliquotType::Terminating)
        } else if divsum > limit {
            Some(AliquotType::NonTerminating)
        } else if let Some(prev_idx) = terms.iter().position(|&x| x == divsum) {
            let cycle_len = terms.len() - prev_idx;
            Some(if prev_idx == 0 {
                match cycle_len {
                    1 => AliquotType::Perfect,
                    2 => AliquotType::Amicable,
                    _ => AliquotType::Sociable,
                }
            } else if cycle_len == 1 {
                AliquotType::Aspiring
            } else {
                AliquotType::Cyclic

            })
        } else {
            None
        };
        terms.push(divsum);
        if let Some(result) = classification {
            return (result, terms);
        }
    }
    (AliquotType::NonTerminating, terms)
}

fn main() {
    let nums = [1i64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 28, 496, 220, 1184, 12496, 1264460, 790,
                909, 562, 1064, 1488 /* , 15355717786080 */];
    for num in &nums {
        println!("{} {:?}", num, classify_aliquot(*num));
    }
}

#[cfg(test)]
mod tests {
    use super::{AliquotType, classify_aliquot};

    #[test]
    fn terminating() {
        assert_eq!(classify_aliquot(11),
                   (AliquotType::Terminating, vec![11, 1, 0]));
        assert_eq!(classify_aliquot(12),
                   (AliquotType::Terminating, vec![12, 16, 15, 9, 4, 3, 1, 0]));
    }

    #[test]
    fn perfect() {
        assert_eq!(classify_aliquot(28), (AliquotType::Perfect, vec![28, 28]));
        assert_eq!(classify_aliquot(496),
                   (AliquotType::Perfect, vec![496, 496]));
    }

    #[test]
    fn amicable() {
        assert_eq!(classify_aliquot(220),
                   (AliquotType::Amicable, vec![220, 284, 220]));
        assert_eq!(classify_aliquot(1184),
                   (AliquotType::Amicable, vec![1184, 1210, 1184]));
    }

    #[test]
    fn sociable() {
        assert_eq!(classify_aliquot(12_496),
                   (AliquotType::Sociable, vec![12_496, 14_288, 15_472, 14_536, 14_264, 12_496]));
        assert_eq!(classify_aliquot(1_264_460),
                   (AliquotType::Sociable,
                    vec![1_264_460, 1_547_860, 1_727_636, 1_305_184, 1_264_460]));
    }

    #[test]
    fn aspiring() {
        assert_eq!(classify_aliquot(790),
                   (AliquotType::Aspiring, vec![790, 650, 652, 496, 496]));
        assert_eq!(classify_aliquot(909),
                   (AliquotType::Aspiring, vec![909, 417, 143, 25, 6, 6]));
    }

    #[test]
    fn cyclic() {
        assert_eq!(classify_aliquot(562),
                   (AliquotType::Cyclic, vec![562, 284, 220, 284]));
        assert_eq!(classify_aliquot(1064),
                   (AliquotType::Cyclic, vec![1064, 1336, 1184, 1210, 1184]));
    }

    #[ignore]
    #[test]
    fn non_terminating() {
        assert_eq!(classify_aliquot(1488),
                   (AliquotType::NonTerminating,
                    vec![1488, 2480, 3472, 4464, 8432, 9424, 10_416, 21_328, 22_320, 55_056,
                         95_728, 96_720, 236_592, 459_792, 881_392, 882_384]));
        assert_eq!(classify_aliquot(15_355_717_786_080),
                   (AliquotType::NonTerminating,
                    vec![15_355_717_786_080, 44_534_663_601_120, 144_940_087_464_480]));
    }
}
