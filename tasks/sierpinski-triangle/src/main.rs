use std::iter::repeat;

fn sierpinski(order: usize) {
    let mut triangle = vec!["*".to_string()];
    for i in 0..order {
        let space = repeat(' ').take(2_usize.pow(i as u32)).collect::<String>();

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
    sierpinski(4);

    let order = 4;
    let height = 1 << order;
    let mut state: Vec<bool> = repeat(true).take(height + 1).collect();

    // Compute the triangle line-by-line by viewing it as Pascal's triangle (mod 2)
    for i in 0..height {
        for _ in 0..height - i - 1 {
            print!(" ");
        }

        for filled in state.iter().take(i + 1) {
            let fill = if *filled { "*" } else { " " };

            print!(" {}", fill);
        }

        // Compute the next line
        for j in (i as i32..0).rev().step_by(1) {
            state[j as usize] ^= state[(j - 1) as usize];
        }

        println!();
    }
}
