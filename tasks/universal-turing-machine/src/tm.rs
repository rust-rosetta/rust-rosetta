use std::{
    collections::VecDeque,
    fmt::{Display, Formatter, Result},
};

use crate::{Direction, Rule};

pub struct TM<'a> {
    state: &'a str,
    terminating_states: Vec<&'a str>,
    rules: Vec<Rule<'a>>,
    band: VecDeque<char>,
    head: usize,
    blank: char,
}

impl<'a> TM<'a> {
    pub fn new(
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

    pub fn is_done(&self) -> bool {
        self.terminating_states.contains(&self.state)
    }

    pub fn step(&mut self) {
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

    pub fn band(&self) -> &VecDeque<char> {
        &self.band
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
