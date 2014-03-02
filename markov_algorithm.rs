
struct MarkovRule {
    pattern: ~str,
    replacement: ~str,
    stop: bool
}

impl MarkovRule {
    fn new(pattern: ~str, replacement: ~str, stop: bool) -> MarkovRule {
        MarkovRule {pattern: pattern, replacement: replacement, stop: stop}
    }
}

struct MarkovAlgorithm {
    rules: ~[MarkovRule]
}

impl MarkovAlgorithm {
    fn build_from_string(s: &str) -> Option<MarkovAlgorithm> {
        let mut rules: ~[MarkovRule] = ~[];
        for line in s.lines() {
            // skip comment lines
            if line.char_at(0) == '#'
            {
                continue;
            }
            
            // check for -> (must be preceded by whitespace)
            // invalid ruleset if absent
            // whitespace rules mean there's 2 possible variations: " ->" and "\t->"
            let arrow_pos = line.find_str(" ->").or_else(||{line.find_str("\t->")});
            match arrow_pos {
                None => {
                    // Ruleset is invalid
                    println!("Invalid rule \"{}\"", line);
                    return None
                }
                Some(arrow) => {
                    // extract pattern (trim trailing whitespace)
                    let pattern = line.slice_to(arrow).trim_right();
                    
                    // get the string after the arrow
                    // this adds 3 to skip the arrow itself
                    let line_end = line.slice_from(arrow + 3).trim_left();
                    
                    // check for . (stop)
                    let stop = (line_end.char_at(0) == '.');
                    
                    // extract replacement
                    let replacement = if stop {line_end.slice_from(1)} else {line_end};
                    
                    // add to rules
                    let new_rule = MarkovRule::new(pattern.to_owned(), replacement.to_owned(), stop);
                    rules.push(new_rule);
                }
            }
        }
        let rule_set = MarkovAlgorithm{rules: rules};
        Some(rule_set)
    }
    
    fn apply(self, input: &str) -> ~str {
        drop(input);
        ~"nope"
    }
}

fn main() {
    let a =
"somehing -> .nothing
#comment
nothing -> oh noes";

    let markov_algorithm = MarkovAlgorithm::build_from_string(a);
}
