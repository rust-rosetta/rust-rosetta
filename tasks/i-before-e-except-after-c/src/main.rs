use std::fs::File;
use std::io::Read;
use std::path::Path;

enum MatchState {
    /// Nothing of interest seen so far
    Nothing,

    /// Last seen a 'c'
    C,

    /// Last seen a 'c' followed by an 'e'
    Ce,

    /// Last seen a 'c' followed by an 'i'
    Ci,

    /// Last seen an 'e' not preceded by a 'c'
    E,

    /// Last seen an 'i' not preceded by a 'c'
    I,
}

struct Occurrences {
    cie: u32,
    cei: u32,
    ie: u32,
    ei: u32,
}

fn count_occurrences(data: &str) -> Occurrences {
    // The counting process is implemented by a state machine. The state variable
    // tracks what pattern prefix was recognized so far (details at MatchState).
    // Each time a full pattern is matched the corresponding saw_* variable is set
    // to true to record its presence for the current word (They are not added
    // directly to result to ensure that words having multiple occurrences of one
    // pattern are only counted once.).
    // At each word boundary add to result what was recorded and clear all state
    // for next word.
    let mut result = Occurrences {
        cie: 0,
        cei: 0,
        ie: 0,
        ei: 0,
    };
    let mut saw_cie = false;
    let mut saw_cei = false;
    let mut saw_ie = false;
    let mut saw_ei = false;
    let mut state = MatchState::Nothing;
    for c in data.chars() {
        state = match (state, c.to_lowercase().next().unwrap()) {
            (_, '\n') | (_, '\r') => {
                if saw_cie {
                    result.cie += 1;
                    saw_cie = false;
                }
                if saw_cei {
                    result.cei += 1;
                    saw_cei = false;
                }
                if saw_ie {
                    result.ie += 1;
                    saw_ie = false;
                }
                if saw_ei {
                    result.ei += 1;
                    saw_ei = false;
                }
                MatchState::Nothing
            }
            (_, 'c') => MatchState::C,
            (MatchState::C, 'i') => MatchState::Ci,
            (MatchState::Ce, 'i') => {
                saw_cei = true;
                MatchState::Nothing
            }
            (MatchState::E, 'i') => {
                saw_ei = true;
                MatchState::Nothing
            }
            (_, 'i') => MatchState::I,
            (MatchState::C, 'e') => MatchState::Ce,
            (MatchState::Ci, 'e') => {
                saw_cie = true;
                MatchState::Nothing
            }
            (MatchState::I, 'e') => {
                saw_ie = true;
                MatchState::Nothing
            }
            (_, 'e') => MatchState::E,
            _ => MatchState::Nothing,
        };
    }
    result
}

fn main() {
    let path = Path::new("resources/unixdict.txt");
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let occ = count_occurrences(&data);
    println!("I before E when not preceded by C is {} (ie: {}, cie: {})",
             if occ.ie > 2 * occ.cie {
                 "plausible"
             } else {
                 "implausible"
             },
             occ.ie,
             occ.cie);
    println!("E before I when preceded by C is {} (cei: {}, ei: {})",
             if occ.cei > 2 * occ.ei {
                 "plausible"
             } else {
                 "implausible"
             },
             occ.cei,
             occ.ei);
}

#[test]
fn basic_test() {
    let occ1 = count_occurrences("ceiling\nclient\nleisure\n");
    assert_eq!(occ1.cie, 0);
    assert_eq!(occ1.cei, 1);
    assert_eq!(occ1.ie, 1);
    assert_eq!(occ1.ei, 1);
}
