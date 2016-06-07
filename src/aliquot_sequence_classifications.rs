// http://rosettacode.org/wiki/Aliquot_sequence_classifications

#[derive(Debug)]
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
