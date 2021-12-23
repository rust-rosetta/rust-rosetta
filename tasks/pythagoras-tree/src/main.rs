//Creates a pythagoras_tree.svg file (12 levels) that can be opened in a browser

fn main() {
    let mut s = " xmlns='http://www.w3.org/2000/svg' stroke='#FFFFFF'>".to_string();
    let mut p0 = (-200f64, 0f64);
    let mut lvl_base = vec![[p0, (-p0.0, 0.0)]];
    for lvl in 0u8..12 {
        s += &format!(
            "<g fill='#{:02X}{:02X}18'>", // level color
            0x28_u8.wrapping_add(lvl.wrapping_mul(20)),
            0x18_u8.wrapping_add(lvl.wrapping_mul(30))
        );
        let mut next_base = Vec::new();
        for [a, b] in lvl_base {
            let xx = b.0 - a.0;
            let yy = b.1 - a.1;
            let c = (a.0 + yy, a.1 - xx);
            let d = (b.0 + yy, b.1 - xx);
            let e = (c.0 + 0.5 * (xx + yy), c.1 - 0.5 * (xx - yy));
            p0 = ([c, d, e].iter()).fold(p0, |(x0, y0), &(x, y)| (x0.min(x), y0.min(y)));
            s += "<polygon points='";
            ([a, c, e, d, c, d, b].iter()).for_each(|(x, y)| s += &format!(" {:.0} {:.0}", x, y));
            s += "'></polygon>";
            next_base.push([c, e]);
            next_base.push([e, d]);
        }
        s += "</g>";
        lvl_base = next_base;
    }
    s = format!("<svg viewBox='{} {} {} {}'", p0.0, p0.1, -p0.0 * 2.0, -p0.1) + &s + "</svg>";

    match std::fs::write("pythagoras_tree.svg", s) {
        Ok(()) => println!("pythagoras_tree.svg file written successfully!"),
        Err(e) => println!("failed to write pythagoras_tree.svg: {}", e),
    }
}
