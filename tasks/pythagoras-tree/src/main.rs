//Creates a pythagoras_tree.svg file (12 levels) that can be opened in a browser

fn main() {
    let mut s = " xmlns='http://www.w3.org/2000/svg' stroke='#FFFFFF'>".to_string();
    let mut p: (f64, f64) = (-200.0, 0.0);
    let mut lvl_base = vec![[p, (-p.0, p.1)]];
    for lvl in 0u8..12 {
        let rg = |start, step| lvl.wrapping_mul(step).wrapping_add(start);
        s += &format!("<g fill='#{:02X}{:02X}18'>", rg(0x28, 20), rg(0x18, 30)); // level color
        let build_segment = |[a, b]: [(f64, f64); 2]| {
            let v = (b.0 - a.0, b.1 - a.1);
            let c = (a.0 + v.1, a.1 - v.0);
            let d = (c.0 + v.0, c.1 + v.1);
            let e = (c.0 + 0.5 * (v.0 + v.1), c.1 + 0.5 * (v.1 - v.0));
            p = ([c, d, e].iter()).fold(p, |(p0, p1), (x, y)| (x.min(p0), y.min(p1)));
            s += "<polygon points='";
            ([a, c, e, d, c, d, b].iter()).for_each(|(x, y)| s += &format!(" {:.0} {:.0}", x, y));
            s += "'></polygon>";
            [[c, e], [e, d]]
        };
        lvl_base = lvl_base.into_iter().flat_map(build_segment).collect();
        s += "</g>";
    }
    s = format!("<svg viewBox='{} {} {} {}'", p.0, p.1, -p.0 * 2.0, -p.1) + &s + "</svg>";

    match std::fs::write("pythagoras_tree.svg", s) {
        Ok(()) => println!("pythagoras_tree.svg file written successfully!"),
        Err(e) => println!("failed to write pythagoras_tree.svg: {}", e),
    }
}
