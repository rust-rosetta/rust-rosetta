//Creates a pythagoras_tree.svg file (12 levels) that can be opened in a browser

use svg::node::element::{Group, Polygon};

fn main() {
    let mut doc = svg::Document::new().set("stroke", "white");
    let mut base: Vec<[(f64, f64); 2]> = vec![[(-200.0, 0.0), (200.0, 0.0)]];
    for lvl in 0..12u8 {
        let r = lvl.wrapping_mul(20).wrapping_add(0x28);
        let g = lvl.wrapping_mul(30).wrapping_add(0x18);
        let mut group = Group::new().set("fill", format!("#{:02X}{:02X}18", r, g)); // level color
        let mut next_base = Vec::new();
        for [a, b] in base {
            let v = (b.0 - a.0, b.1 - a.1);
            let c = (a.0 + v.1, a.1 - v.0);
            let d = (c.0 + v.0, c.1 + v.1);
            let e = (c.0 + 0.5 * (v.0 + v.1), c.1 + 0.5 * (v.1 - v.0));
            group = group.add(Polygon::new().set("points", vec![a, c, e, d, c, d, b]));
            next_base.extend([[c, e], [e, d]]);
        }
        base = next_base;
        doc = doc.add(group);
    }
    let (x0, y0) = (base.iter()).fold((0.0, 0.0), |(x0, y0), [(x, y), _]| (x.min(x0), y.min(y0)));
    let path = "pythagoras_tree.svg";
    match svg::save(path, &doc.set("viewBox", (x0, y0, -x0 * 2.0, -y0))) {
        Ok(_) => println!("{} file written successfully!", path),
        Err(e) => println!("failed to write {}: {}", path, e),
    }
}
