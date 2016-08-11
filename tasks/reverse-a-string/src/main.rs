extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "一二三四五六七八九十";
    let s2 = "as⃝df̅";
    let reversed: String = s.chars().rev().collect();
    let reversed2: String = UnicodeSegmentation::graphemes(s2, true).rev().collect();
    println!("{}", reversed);
    println!("{}", reversed2);
}
