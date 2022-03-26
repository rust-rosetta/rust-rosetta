use crate::Direction;

pub struct Rule<'a> {
    pub state: &'a str,
    pub read: char,
    pub write: char,
    pub dir: Direction,
    pub new_state: &'a str,
}

impl<'a> Rule<'a> {
    pub fn new(
        state: &'a str,
        read: char,
        write: char,
        dir: Direction,
        new_state: &'a str,
    ) -> Self {
        Self {
            state,
            read,
            write,
            dir,
            new_state,
        }
    }
}
