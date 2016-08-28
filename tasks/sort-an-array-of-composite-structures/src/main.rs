#[derive(Debug, PartialEq)]
pub struct Element {
    name: String,
    value: String,
}

impl Element {
    fn new(name: &str, value: &str) -> Element {
        Element {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

pub fn sort_by_name(elements: &mut Vec<Element>) {
    elements.sort_by(|a, b| a.name.cmp(&b.name));
}

fn main() {
    let mut values = vec![
        Element::new("Iron", "Fe"),
        Element::new("Cobalt", "Co"),
        Element::new("Nickel", "Ni"),
        Element::new("Copper", "Cu"),
        Element::new("Zinc", "Zn"),
    ];
    sort_by_name(&mut values);
    println!("{:?}", values);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let mut values = vec![
            Element::new("Iron", "Fe"),
            Element::new("Cobalt", "Co"),
            Element::new("Nickel", "Ni"),
            Element::new("Copper", "Cu"),
            Element::new("Zinc", "Zn"),
        ];
        sort_by_name(&mut values);
        assert_eq!(values,
                   vec![
            Element::new("Cobalt", "Co"),
            Element::new("Copper", "Cu"),
            Element::new("Iron", "Fe"),
            Element::new("Nickel", "Ni"),
            Element::new("Zinc", "Zn"),
        ]);
    }
}
