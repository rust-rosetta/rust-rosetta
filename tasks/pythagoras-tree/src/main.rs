// Creates a pythagoras_tree.svg file that can be opened in a browser

fn main() {
    let mut s = String::new();
    let mut vbox = [(0f64, 0f64); 2];
    let mut rg = 0xF0;

    let next_level = |vpp: &Vec<[(f64, f64); 2]>| {
        s += &format!("<g fill='#{:02X}{:02X}10'>", 0xA8 - rg / 2, 0xFF - rg); // level color
        let mut next_vpp = Vec::new();
        for &[a, b] in vpp {
            let xx = b.0 - a.0;
            let yy = b.1 - a.1;
            let c = (a.0 + yy, a.1 - xx);
            let d = (b.0 + yy, b.1 - xx);
            let e = (c.0 + 0.5 * (xx + yy), c.1 - 0.5 * (xx - yy));
            vbox = [c, d, e].iter().fold(vbox, |[mi, ma], (x, y)| {
                [(x.min(mi.0), y.min(mi.1)), (x.max(ma.0), y.max(ma.1))]
            });
            s += "<polygon points='";
            ([a, c, e, d, c, d, b].iter()).for_each(|(x, y)| s += &format!(" {:.0} {:.0}", x, y));
            s += "'></polygon>";
            next_vpp.push([c, e]);
            next_vpp.push([e, d]);
        }
        s += "</g>";
        rg = rg * 3 / 4;
        Some(next_vpp)
    };
    std::iter::successors(Some(vec![[(0.0, 0.0), (500.0, 0.0)]]), next_level).nth(6);
    let attr = format!("viewBox='{:.0} {:.0} {:.0} {:.0}' stroke='#FFFFFF' xmlns='http://www.w3.org/2000/svg' xmlns:ev='http://www.w3.org/2001/xml-events'", 
    vbox[0].0, vbox[0].1, vbox[1].0 - vbox[0].0, vbox[1].1 - vbox[0].1);
    std::fs::write("pythagoras_tree.svg", format!("<svg {}>{}</svg>", attr, s)).unwrap();
}
