// For abstraction, it is desirable to implement the transitions of the state machine through its methods.
// Here it is done transparently using the method_enum::gen macro.

enum State {
    Ready,
    Waiting,
    Dispense,
    Refunding,
    Exit,
}

#[methods_enum::gen(Act: run)]
impl State {
    pub fn set(&mut self);
    pub fn input_char(&mut self, ch: char);

    fn run(&mut self, act: Act) {
        match self {
            State::Ready => match act {
                Act::set() => println!("Ready: d - deposit / q - quit "),
                Act::input_char('d') => self.set_state(State::Waiting),
                Act::input_char('q') => self.set_state(State::Exit),
                _ => self.set(),
            },
            State::Waiting => match act {
                Act::set() => println!("Waiting: s - select / r - refund "),
                Act::input_char('s') => self.set_state(State::Dispense),
                Act::input_char('r') => self.set_state(State::Refunding),
                _ => self.set(),
            },
            State::Dispense => match act {
                Act::set() => println!("Dispense: r - remove "),
                Act::input_char('r') => self.set_state(State::Ready),
                _ => self.set(),
            },
            State::Refunding => match act {
                Act::set() => {
                    println!("Refunding: refund of the deposit...");
                    self.set_state(State::Ready)
                }
                _ => (), // never - ignore
            },
            State::Exit => match act {
                Act::set() => println!("Exit: goodbye! "),
                _ => panic!("!! Invalid command for State::Exit: '{act:?}'"),
            },
        }
    }

    fn set_state(&mut self, new_state: State) {
        *self = new_state;
        self.set();
    }
}

fn main() {
    let mut machine = State::Ready;
    machine.set();

    while !matches!(&machine, State::Exit) {
        machine.input_char(char_entered());
    }
}

fn char_entered() -> char {
    let mut text = String::new();
    (std::io::stdin().read_line(&mut text)).unwrap_or(0);
    text.chars().next().unwrap_or('\x0d')
}
