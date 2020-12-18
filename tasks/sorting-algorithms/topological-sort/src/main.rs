use std::boxed::Box;
use std::collections::{HashMap, HashSet};
use std::env;

/// model a VHDL library
#[derive(Debug, PartialEq, Eq, Hash)]
struct Library<'a> {
    name: &'a str,
    children: Vec<&'a str>,
    num_parents: usize,
}

/// transform raw input into a DAG structure for later sorting
fn build_libraries(input: Vec<&str>) -> HashMap<&str, Box<Library>> {
    let mut libraries: HashMap<&str, Box<Library>> = HashMap::new();

    for input_line in input {
        let line_split = input_line.split_whitespace().collect::<Vec<&str>>();
        let name = line_split.get(0).unwrap();
        let mut num_parents: usize = 0;
        for parent in line_split.iter().skip(1) {
            if parent == name {
                continue;
            }
            if !libraries.contains_key(parent) {
                libraries.insert(
                    parent,
                    Box::new(Library {
                        name: parent,
                        children: vec![name],
                        num_parents: 0,
                    }),
                );
            } else {
                libraries.get_mut(parent).unwrap().children.push(name);
            }
            num_parents += 1;
        }

        if !libraries.contains_key(name) {
            libraries.insert(
                name,
                Box::new(Library {
                    name,
                    children: Vec::new(),
                    num_parents,
                }),
            );
        } else {
            libraries.get_mut(name).unwrap().num_parents = num_parents;
        }
    }
    libraries
}

/// an implementation of Kahn's algorithm for topological sorting
/// It will correctly detect cycles too.
fn topological_sort<'a>(
    mut libraries: HashMap<&'a str, Box<Library<'a>>>,
) -> Result<Vec<&'a str>, String> {
    let mut needs_processing = libraries
        .iter()
        .map(|(k, _v)| <&str>::clone(k))
        .collect::<HashSet<&str>>();
    let mut options: Vec<&str> = libraries
        .iter()
        .filter(|(_k, v)| v.num_parents == 0)
        .map(|(k, _v)| *k)
        .collect();
    let mut sorted: Vec<&str> = Vec::new();
    while !options.is_empty() {
        let cur = options.pop().unwrap();
        for children in libraries
            .get_mut(cur)
            .unwrap()
            .children
            .drain(0..)
            .collect::<Vec<&str>>()
        {
            let child = libraries.get_mut(children).unwrap();
            child.num_parents -= 1;
            if child.num_parents == 0 {
                options.push(child.name)
            }
        }
        sorted.push(cur);
        needs_processing.remove(cur);
    }
    match needs_processing.is_empty() {
        true => Ok(sorted),
        false => Err(format!("Cycle detected among {:?}", needs_processing)),
    }
}

/// Execute `cargo run` in your shell to print the sorted output.
/// To run it with a cycle, execute `cargo run -- cycle`.
fn main() {
    // see if the user wants to see the sort detect a cycle
    let add_cycle = env::args().nth(1).is_some();
    let mut input: Vec<&str> = vec![
        "des_system_lib   std synopsys std_cell_lib des_system_lib dw02 dw01 ramlib ieee\n",
        "dw01             ieee dw01 dware gtech \n",
        "dw02             ieee dw02 dware\n",
        "dw03             std synopsys dware dw03 dw02 dw01 ieee gtech\n",
        "dw04             dw04 ieee dw01 dware gtech\n",
        "dw05             dw05 ieee dware\n",
        "dw06             dw06 ieee dware\n",
        "dw07             ieee dware\n",
        "dware            ieee dware\n",
        "gtech            ieee gtech\n",
        "ramlib           std ieee\n",
        "std_cell_lib     ieee std_cell_lib\n",
        "synopsys\n",
    ];
    if add_cycle {
        input[1] = "dw01             ieee dw01 dware gtech dw04\n";
    }

    let libraries = build_libraries(input);
    match topological_sort(libraries) {
        Ok(sorted) => println!("{:?}", sorted),
        Err(msg) => println!("{:?}", msg),
    }
}
