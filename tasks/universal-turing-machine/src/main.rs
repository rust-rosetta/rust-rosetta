use universal_turing_machine::{Direction, Rule, TM};

fn main() {
    println!("Simple incrementer");
    let mut tm_si = TM::new(
        vec!["q0", "qf"],
        "q0",
        vec!["qf"],
        vec!['B', '1'],
        'B',
        vec![
            Rule::new("q0", '1', '1', Direction::Right, "q0"),
            Rule::new("q0", 'B', '1', Direction::Stay, "qf"),
        ],
        "111",
    );
    while !tm_si.is_done() {
        println!("{}", tm_si);
        tm_si.step();
    }

    println!("___________________");
    println!("Three-state busy beaver");
    let mut tm_bb3 = TM::new(
        vec!["a", "b", "c", "halt"],
        "a",
        vec!["halt"],
        vec!['0', '1'],
        '0',
        vec![
            Rule::new("a", '0', '1', Direction::Right, "b"),
            Rule::new("a", '1', '1', Direction::Left, "c"),
            Rule::new("b", '0', '1', Direction::Left, "a"),
            Rule::new("b", '1', '1', Direction::Right, "b"),
            Rule::new("c", '0', '1', Direction::Left, "b"),
            Rule::new("c", '1', '1', Direction::Stay, "halt"),
        ],
        "0",
    );
    while !tm_bb3.is_done() {
        println!("{}", tm_bb3);
        tm_bb3.step();
    }
    println!("{}", tm_bb3);

    println!("___________________");
    println!("Five-state busy beaver");
    let mut tm_bb5 = TM::new(
        vec!["A", "B", "C", "D", "E", "H"],
        "A",
        vec!["H"],
        vec!['0', '1'],
        '0',
        vec![
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
        ],
        "0",
    );
    let mut steps = 0;
    while !tm_bb5.is_done() {
        tm_bb5.step();
        steps += 1;
    }
    println!("Steps: {}", steps);
    println!("Band length: {}", tm_bb5.band().len());
}
