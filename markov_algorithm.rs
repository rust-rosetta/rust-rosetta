
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
    pub fn build_from_string(s: &str) -> Option<MarkovAlgorithm> {
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
    
    pub fn apply(&self, input: &str) -> ~str {
        let mut state = input.into_owned();
        
        // Don't allow input to be used after this
        drop(input);
        
        // loop while operations are possible
        loop {
            // find the first rule that is applicable
            // (pattern string is in state)
            let mut rule_iterator = self.rules.iter();
            let possible_rule = rule_iterator.find(|rule|{
                state.find_str(rule.pattern).is_some()
            });
            
            match possible_rule {
                // stop if no rule found
                None => { break; }
                Some(rule) => {
                    // replace the first instnace (only) of the pattern
                    // Note: cannot use str::replace as that replaces all instances
                    
                    // unwrap is safe here as the code for finding a rule
                    // already established that the pattern is present
                    let pos = state.find_str(rule.pattern).unwrap();
                    let width = rule.pattern.len();
                    
                    // string parts
                    let left = state.slice_to(pos).to_owned();
                    let right = state.slice_from(pos + width).to_owned();
                    
                    // construct new string
                    state = left + rule.replacement + right;
                    
                    // stop if required
                    if (rule.stop) { break; }
                }
            }
        }
        
        state
    }
}

fn main() {
    let a =
~"somehing -> .nothing
#comment
nothing -> oh noes";

    let markov_algorithm = MarkovAlgorithm::build_from_string(a);
    
    let output = markov_algorithm.map(|algorithm|{
        algorithm.apply("something")
    });
    
    output.map(|output_string|{
        println!("{}", output_string);
    });
}
