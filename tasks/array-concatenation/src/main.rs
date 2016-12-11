fn main() {
    let a_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let b_vec: Vec<i32> = vec![6; 5];

    let c_vec = concatenate_arrays::<i32>(a_vec.as_slice(), b_vec.as_slice());

    println!("{:?} ~ {:?} => {:?}", a_vec, b_vec, c_vec);
}

fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
    let mut concat: Vec<T> = vec![x[0].clone(); x.len()];

    concat.clone_from_slice(x);
    concat.extend_from_slice(y);

    concat
}

#[cfg(test)]
mod tests {
    use super::concatenate_arrays;

    #[derive(Clone, Debug, PartialEq)]
    struct Dummy {
        a: f64,
        b: &'static str,
    }

    #[test]
    fn test_concatenation_int() {
        let a_vec: Vec<u64> = vec![0, 1, 2, 3, 4];
        let b_vec: Vec<u64> = vec![5; 5];
        let c_vec = concatenate_arrays::<u64>(a_vec.as_slice(), b_vec.as_slice());

        assert_eq!(c_vec, [0, 1, 2, 3, 4, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_concatenation_str() {
        let a_vec: Vec<&str> = vec!["hay", "ye", "eye", "owe", "you"];
        let b_vec: Vec<&str> = vec!["why"];
        let c_vec = concatenate_arrays::<&str>(a_vec.as_slice(), b_vec.as_slice());

        assert_eq!(c_vec, ["hay", "ye", "eye", "owe", "you", "why"]);
    }

    #[test]
    fn test_concatenation_tuple() {
        let a_vec: Vec<(i32, &str)> = vec![(0, "hay"), (1, "ye"), (2, "eye")];
        let b_vec: Vec<(i32, &str)> = vec![(3, "owe"), (4, "you")];
        let c_vec = concatenate_arrays::<(i32, &str)>(a_vec.as_slice(), b_vec.as_slice());

        assert_eq!(c_vec,
                   [(0, "hay"), (1, "ye"), (2, "eye"), (3, "owe"), (4, "you")]);
    }

    #[test]
    fn test_concatenation_struct() {
        let a_vec: Vec<Dummy> = vec![Dummy { a: 0.0, b: "hay" },
                                     Dummy { a: 1.1, b: "ye" },
                                     Dummy { a: 2.2, b: "eye" }];
        let b_vec: Vec<Dummy> = vec![Dummy { a: 3.3, b: "owe" }, Dummy { a: 4.4, b: "you" }];

        let c_vec = concatenate_arrays::<Dummy>(a_vec.as_slice(), b_vec.as_slice());

        assert_eq!(c_vec,
                   [Dummy { a: 0.0, b: "hay" },
                    Dummy { a: 1.1, b: "ye" },
                    Dummy { a: 2.2, b: "eye" },
                    Dummy { a: 3.3, b: "owe" },
                    Dummy { a: 4.4, b: "you" }]);
    }
}
