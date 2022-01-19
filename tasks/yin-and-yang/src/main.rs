// Creates a yin_yang.svg file. Rust version 1.58.0 or higher required

use svg::node::element::Path;

fn main() {
    let doc = svg::Document::new()
        .add(yin_yang(15.0, 1.0).set("transform", "translate(20,20)"))
        .add(yin_yang(6.0, 1.0).set("transform", "translate(50,11)"));
    svg::save("yin_yang.svg", &doc).unwrap();
}
/// th - the thickness of the outline around yang
fn yin_yang(r: f32, th: f32) -> Path {
    let (cr, cw, ccw) = (",0,1,1,.1,0z", ",0,0,1,0,", ",0,0,0,0,");
    let d = format!("M0,{0} a{0},{0}{cr} M0,{1} ", r + th, -r / 3.0)
        + &format!("a{0},{0}{cr} m0,{r} a{0},{0}{cr} M0,0 ", r / 6.0)
        + &format!("A{0},{0}{ccw}{r} A{r},{r}{cw}-{r} A{0},{0}{cw}0", r / 2.0); // main_circle + eyes + yang
    Path::new().set("d", d).set("fill-rule", "evenodd")
}
