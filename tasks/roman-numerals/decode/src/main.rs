struct RomanNumeral {
    symbol: &'static str,
    value: u32,
}

const NUMERALS: [RomanNumeral; 13] = [RomanNumeral {
                                          symbol: "M",
                                          value: 1000,
                                      },
                                      RomanNumeral {
                                          symbol: "CM",
                                          value: 900,
                                      },
                                      RomanNumeral {
                                          symbol: "D",
                                          value: 500,
                                      },
                                      RomanNumeral {
                                          symbol: "CD",
                                          value: 400,
                                      },
                                      RomanNumeral {
                                          symbol: "C",
                                          value: 100,
                                      },
                                      RomanNumeral {
                                          symbol: "XC",
                                          value: 90,
                                      },
                                      RomanNumeral {
                                          symbol: "L",
                                          value: 50,
                                      },
                                      RomanNumeral {
                                          symbol: "XL",
                                          value: 40,
                                      },
                                      RomanNumeral {
                                          symbol: "X",
                                          value: 10,
                                      },
                                      RomanNumeral {
                                          symbol: "IX",
                                          value: 9,
                                      },
                                      RomanNumeral {
                                          symbol: "V",
                                          value: 5,
                                      },
                                      RomanNumeral {
                                          symbol: "IV",
                                          value: 4,
                                      },
                                      RomanNumeral {
                                          symbol: "I",
                                          value: 1,
                                      }];

fn to_hindu(roman: &str) -> u32 {
    match NUMERALS.iter().find(|num| roman.starts_with(num.symbol)) {
        Some(num) => num.value + to_hindu(&roman[num.symbol.len()..]),
        None => 0, // if string empty, add nothing
    }
}

fn main() {
    let roms = ["MMXIV", "MCMXCIX", "XXV", "MDCLXVI", "MMMDCCCLXXXVIII"];
    for &r in &roms {
        // 15 is minimum formatting width of the first argument, there for alignment
        println!("{:2$} = {}", r, to_hindu(r), 15);
    }
}
