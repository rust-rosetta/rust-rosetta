use std::cmp::Ordering;

struct Table {
    rows: Vec<Vec<String>>,
    ordering_function: fn(&str, &str) -> Ordering,
    ordering_column: usize,
    reverse: bool,
}

impl Table {
    fn new(rows: Vec<Vec<String>>) -> Table {
        Table {
            rows: rows,
            ordering_column: 0,
            reverse: false,
            ordering_function: |str1, str2| str1.cmp(str2),
        }
    }
}

impl Table {
    fn with_ordering_column(&mut self, ordering_column: usize) -> &mut Table {
        self.ordering_column = ordering_column;
        self
    }

    fn with_reverse(&mut self, reverse: bool) -> &mut Table {
        self.reverse = reverse;
        self
    }

    fn with_ordering_fun(&mut self, compare: fn(&str, &str) -> Ordering) -> &mut Table {
        self.ordering_function = compare;
        self
    }

    fn sort(&mut self) {
        let fun = &mut self.ordering_function;
        let idx = self.ordering_column;
        if self.reverse {
            self.rows
                .sort_unstable_by(|vec1, vec2| (fun)(&vec1[idx], &vec2[idx]).reverse());
        } else {
            self.rows
                .sort_unstable_by(|vec1, vec2| (fun)(&vec1[idx], &vec2[idx]));
        }
    }
}

#[cfg(test)]
mod test {
    use super::Table;

    fn generate_test_table() -> Table {
        Table::new(vec![
            vec!["0".to_string(), "fff".to_string()],
            vec!["2".to_string(), "aab".to_string()],
            vec!["1".to_string(), "ccc".to_string()],
        ])
    }

    #[test]
    fn test_simple_sort() {
        let mut table = generate_test_table();
        table.sort();
        assert_eq!(
            table.rows,
            vec![
                vec!["0".to_string(), "fff".to_string()],
                vec!["1".to_string(), "ccc".to_string()],
                vec!["2".to_string(), "aab".to_string()],
            ],
        )
    }

    #[test]
    fn test_ordering_column() {
        let mut table = generate_test_table();
        table.with_ordering_column(1).sort();
        assert_eq!(
            table.rows,
            vec![
                vec!["2".to_string(), "aab".to_string()],
                vec!["1".to_string(), "ccc".to_string()],
                vec!["0".to_string(), "fff".to_string()],
            ],
        )
    }

    #[test]
    fn test_with_reverse() {
        let mut table = generate_test_table();
        table.with_reverse(true).sort();
        assert_eq!(
            table.rows,
            vec![
                vec!["2".to_string(), "aab".to_string()],
                vec!["1".to_string(), "ccc".to_string()],
                vec!["0".to_string(), "fff".to_string()],
            ],
        )
    }

    #[test]
    fn test_custom_ordering_fun() {
        let mut table = generate_test_table();
        // Simple ordering function that reverses stuff.
        // Should operate like the test before.
        table.with_ordering_fun(|x, y| x.cmp(y).reverse()).sort();
        assert_eq!(
            table.rows,
            vec![
                vec!["2".to_string(), "aab".to_string()],
                vec!["1".to_string(), "ccc".to_string()],
                vec!["0".to_string(), "fff".to_string()],
            ],
        )
    }

    #[test]
    fn test_everything_together() {
        let mut table = generate_test_table();
        // Using the reversing cmp function, then reverse (= don't do anything)
        // then sort from column 1.
        table
            .with_ordering_fun(|x, y| x.cmp(y).reverse())
            .with_reverse(true)
            .with_ordering_column(1)
            .sort();
        assert_eq!(
            table.rows,
            vec![
                vec!["2".to_string(), "aab".to_string()],
                vec!["1".to_string(), "ccc".to_string()],
                vec!["0".to_string(), "fff".to_string()],
            ],
        )
    }
}
