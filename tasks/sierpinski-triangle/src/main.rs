fn sierpinski(order: usize) {
    let mut triangle = vec!["*".to_string()];
    for i in 0..order {
        #[allow(clippy::cast_possible_truncation)]
        let space = " ".repeat(2_usize.pow(i as u32));

        // save original state
        let mut d = triangle.clone();

        // extend existing lines
        d.iter_mut().for_each(|r| {
            let new_row = format!("{}{}{}", space, r, space);
            *r = new_row;
        });

        // add new lines
        triangle.iter().for_each(|r| {
            let new_row = format!("{}{}{}", r, " ", r);
            d.push(new_row);
        });

        triangle = d;
    }

    triangle.iter().for_each(|r| println!("{}", r));
}
fn main() {
    let order = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "4".to_string())
        .parse::<usize>()
        .unwrap();

    sierpinski(order);
}
