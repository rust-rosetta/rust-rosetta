use std::collections::HashMap;

fn main() {
    let commands = "
        Add ALTer  BAckup Bottom  CAppend Change SCHANGE  CInsert CLAst COMPress COpy \
        COUnt COVerlay CURsor DELete CDelete Down DUPlicate Xedit EXPand EXTract Find \
        NFind NFINDUp NFUp CFind FINdup FUp FOrward GET Help HEXType Input POWerinput \
        Join SPlit SPLTJOIN  LOAD  Locate CLocate  LOWercase UPPercase  LPrefix MACRO \
        MErge MODify MOve MSG Next Overlay PARSE PREServe PURge PUT PUTD  Query  QUIT \
        READ  RECover REFRESH RENum REPeat  Replace CReplace  RESet  RESTore  RGTLEFT \
        RIght LEft  SAVE  SET SHift SI  SORT  SOS  STAck STATus  TOP TRAnsfer Type Up \
    ";
    let split = commands.split_ascii_whitespace();
    let count_hashtable: HashMap<&str, usize> = split
        .map(|word| {
            (
                word,
                word.chars().take_while(|c| c.is_ascii_uppercase()).count(),
            )
        })
        .collect();

    let line = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
    let mut words_vec: Vec<String> = vec![];
    for word in line.split_ascii_whitespace() {
        let split = commands.split_ascii_whitespace();
        let abbr = split
            .filter(|x| {
                x.to_ascii_lowercase()
                    .starts_with(&word.to_ascii_lowercase())
                    && word.len() >= *count_hashtable.get(x).unwrap()
            })
            .next();
        words_vec.push(match abbr {
            Some(word) => word.to_ascii_uppercase(),
            None => String::from("*error*"),
        });
    }
    let corrected_line = words_vec.join(" ");
    println!("{}", corrected_line);
}
