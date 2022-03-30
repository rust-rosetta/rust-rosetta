use brace_expansion::{expand, tokenize};

fn main() {
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let tokens = tokenize(&input);
    let expanded = expand(tokens);

    for line in &expanded {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"~/{Downloads,Pictures}/*.{jpg,gif,png}";
        let actual = expand(tokenize(input));
        let expected = vec![
            "~/Downloads/*.jpg",
            "~/Downloads/*.gif",
            "~/Downloads/*.png",
            "~/Pictures/*.jpg",
            "~/Pictures/*.gif",
            "~/Pictures/*.png",
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = "It{{em,alic}iz,erat}e{d,}, please.";
        let actual = expand(tokenize(input));
        let expected = vec![
            "Itemized, please.",
            "Itemize, please.",
            "Italicized, please.",
            "Italicize, please.",
            "Iterated, please.",
            "Iterate, please.",
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = r"{,{,gotta have{ ,\, again\, }}more }cowbell!";
        let actual = expand(tokenize(input));
        let expected = vec![
            "cowbell!",
            "more cowbell!",
            "gotta have more cowbell!",
            r"gotta have\, again\, more cowbell!",
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let input = r"{}} some }{,{\\{ edge, edge} \,}{ cases, {here} \\\\\}";
        let actual = expand(tokenize(input));
        let expected = vec![
            r"{}} some }{,{\\ edge \,}{ cases, {here} \\\\\}",
            r"{}} some }{,{\\ edge \,}{ cases, {here} \\\\\}",
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn case5() {
        let input = "a{b{1,2}c";
        let actual = expand(tokenize(input));
        let expected = vec!["a{b1c", "a{b2c"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn case6() {
        let input = r"a{1,2}b}c";
        let actual = expand(tokenize(input));
        let expected = vec!["a1b}c", "a2b}c"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn case7() {
        let input = "a{1,{2},3}b";
        let actual = expand(tokenize(input));
        let expected = vec!["a1b", "a{2}b", "a3b"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn case8() {
        let input = "a{b{1,2}c{}}";
        let actual = expand(tokenize(input));
        let expected = vec!["a{b1c{}}", "a{b2c{}}"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn case9() {
        let input = "more{ darn{ cowbell,},}";
        let actual = expand(tokenize(input));
        let expected = vec!["more darn cowbell", "more darn", "more"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn case10() {
        let input = r"ab{c,d\,e{f,g\h},i\,j{k,l\,m}n,o\,p}qr";
        let actual = expand(tokenize(input));
        let expected = vec![
            "abcqr",
            r"abd\,efqr",
            r"abd\,eg\hqr",
            r"abi\,jknqr",
            r"abi\,jl\,mnqr",
            r"abo\,pqr",
        ];

        assert_eq!(actual, expected);
    }
}
