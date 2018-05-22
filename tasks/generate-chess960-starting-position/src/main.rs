use std::collections::BTreeSet;

struct Chess960(BTreeSet<String>);

impl Chess960 {
    fn invoke(&mut self, b: &str, e: &str) {
        if e.len() <= 1 {
            let s = b.to_string() + e;
            if Chess960::is_valid(&s) {
                self.0.insert(s);
            }
        } else {
            for (i, c) in e.char_indices() {
                let mut b = b.to_string();
                b.push(c);
                let mut e = e.to_string();
                e.remove(i);
                self.invoke(&b, &e);
            }
        }
    }

    fn is_valid(s: &str) -> bool {
        let k = s.find('K').unwrap();
        k > s.find('R').unwrap()
            && k < s.rfind('R').unwrap()
            && s.find('B').unwrap() % 2 != s.rfind('B').unwrap() % 2
    }
}

// Program entry point.
fn main() {
    let mut chess960 = Chess960(BTreeSet::new());
    chess960.invoke("", "KQRRNNBB");

    for (i, p) in chess960.0.iter().enumerate() {
        println!("{}: {}", i, p);
    }
}
