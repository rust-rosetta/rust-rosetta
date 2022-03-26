use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};

fn main() {
    println!("Simple incrementer");
    let rules_si = vec![
        Rule::new("q0", '1', '1', Direction::Right, "q0"),
        Rule::new("q0", 'B', '1', Direction::Stay, "qf"),
    ];
    let states_si = vec!["q0", "qf"];
    let terminating_states_si = vec!["qf"];
    let permissible_symbols_si = vec!['B', '1'];
    let mut tm_si = TM::new(
        states_si,
        "q0",
        terminating_states_si,
        permissible_symbols_si,
        'B',
        rules_si,
        "111",
    );
    while !tm_si.is_done() {
        println!("{}", tm_si);
        tm_si.step();
    }

    println!("___________________");
    println!("Three-state busy beaver");
    let rules_bb3 = vec![
        Rule::new("a", '0', '1', Direction::Right, "b"),
        Rule::new("a", '1', '1', Direction::Left, "c"),
        Rule::new("b", '0', '1', Direction::Left, "a"),
        Rule::new("b", '1', '1', Direction::Right, "b"),
        Rule::new("c", '0', '1', Direction::Left, "b"),
        Rule::new("c", '1', '1', Direction::Stay, "halt"),
    ];
    let states_bb3 = vec!["a", "b", "c", "halt"];
    let terminating_states_bb3 = vec!["halt"];
    let permissible_symbols_bb3 = vec!['0', '1'];
    let mut tm_bb3 = TM::new(
        states_bb3,
        "a",
        terminating_states_bb3,
        permissible_symbols_bb3,
        '0',
        rules_bb3,
        "0",
    );
    while !tm_bb3.is_done() {
        println!("{}", tm_bb3);
        tm_bb3.step();
    }
    println!("{}", tm_bb3);

    println!("___________________");
    println!("Five-state busy beaver");
    let rules_bb5 = vec![
        Rule::new("A", '0', '1', Direction::Right, "B"),
        Rule::new("A", '1', '1', Direction::Left, "C"),
        Rule::new("B", '0', '1', Direction::Right, "C"),
        Rule::new("B", '1', '1', Direction::Right, "B"),
        Rule::new("C", '0', '1', Direction::Right, "D"),
        Rule::new("C", '1', '0', Direction::Left, "E"),
        Rule::new("D", '0', '1', Direction::Left, "A"),
        Rule::new("D", '1', '1', Direction::Left, "D"),
        Rule::new("E", '0', '1', Direction::Stay, "H"),
        Rule::new("E", '1', '0', Direction::Left, "A"),
    ];
    let states_bb5 = vec!["A", "B", "C", "D", "E", "H"];
    let terminating_states_bb5 = vec!["H"];
    let permissible_symbols_bb5 = vec!['0', '1'];
    let mut tm_bb5 = TM::new(
        states_bb5,
        "A",
        terminating_states_bb5,
        permissible_symbols_bb5,
        '0',
        rules_bb5,
        "0",
    );
    let mut steps = 0;
    while !tm_bb5.is_done() {
        tm_bb5.step();
        steps += 1;
    }
    println!("Steps: {}", steps);
    println!("Band lenght: {}", tm_bb5.band.len());
}

struct TM<'a> {
    state: &'a str,
    terminating_states: Vec<&'a str>,
    rules: Vec<Rule<'a>>,
    band: VecDeque<char>,
    head: usize,
    blank: char,
}

struct Rule<'a> {
    state: &'a str,
    read: char,
    write: char,
    dir: Direction,
    new_state: &'a str,
}

enum Direction {
    Left,
    Right,
    Stay,
}

impl<'a> TM<'a> {
    fn new(
        _states: Vec<&'a str>,
        initial_state: &'a str,
        terminating_states: Vec<&'a str>,
        _permissible_symbols: Vec<char>,
        blank: char,
        rules: Vec<Rule<'a>>,
        input: &str,
    ) -> Self {
        Self {
            state: initial_state,
            terminating_states,
            rules,
            band: input.chars().collect::<VecDeque<_>>(),
            head: 0,
            blank,
        }
    }

    fn is_done(&self) -> bool {
        self.terminating_states.contains(&self.state)
    }

    fn step(&mut self) {
        let field = self.band.get(self.head).unwrap();
        let rule = self
            .rules
            .iter()
            .find(|rule| rule.state == self.state && &rule.read == field)
            .unwrap();
        let field = self.band.get_mut(self.head).unwrap();
        *field = rule.write;
        self.state = rule.new_state;
        match rule.dir {
            Direction::Left => {
                if self.head == 0 {
                    self.band.push_front(self.blank)
                } else {
                    self.head -= 1;
                }
            }
            Direction::Right => {
                if self.head == self.band.len() - 1 {
                    self.band.push_back(self.blank)
                }
                self.head += 1;
            }
            Direction::Stay => {}
        }
    }
}

impl<'a> Display for TM<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let band = self
            .band
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i == self.head {
                    format!("[{}]", c)
                } else {
                    format!(" {} ", c)
                }
            })
            .fold(String::new(), |acc, val| acc + &val);
        write!(f, "{}\t{}", self.state, band)
    }
}

impl<'a> Rule<'a> {
    fn new(state: &'a str, read: char, write: char, dir: Direction, new_state: &'a str) -> Self {
        Self {
            state,
            read,
            write,
            dir,
            new_state,
        }
    }
}
