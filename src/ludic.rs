// http://rosettacode.org/wiki/Ludic_numbers

const ARRAY_MAX: usize = 25000;
const LUDIC_MAX: usize = 2100;

fn main() {

    // Variable that will hold all the ludic numbers
    let mut result: Vec<usize> = vec![];

    // Array that will hold the first array with 25.000 numbers starting with number 2
    let mut array = (2..(ARRAY_MAX+2)).collect::<Vec<_>>();

    // Calculate LUDIC_MAX Ludic numbers
    // First Ludic numbers is '1'
    result.push(1);

    for _ in 0..LUDIC_MAX {
        let next_ludic = array[0];
        result.push(next_ludic);
        let mut counter = 0;
        for i in 0..array.len() {
            // modulo operation:
            // ((a % b) + b) % b
            let modulo = ((i as usize % next_ludic) + next_ludic) % next_ludic;

            if modulo != 0 {
                array[counter] = array[i];
                counter += 1;
            }
        }
    }

    print!("First 25: ");
    print_n_ludics(&result, 25);
    println!("");
    print!("Number of Ludics below 1000: ");
    print_num_ludics_upto(&result, 1000);
    println!("");
    print!("Ludics from 2000 to 2005: ");
    print_ludics_from_to(&result, 2000, 2005);
    println!("");
    println!("Triplets below 250: ");
    print_tiples_until(&result, 250);

}

//Function that prints the first 'n' Ludic numbers
fn print_n_ludics(x: &Vec<usize>, n: usize) {
    for i in 0..n {
        print!("{} ", x[i]);
    }
    println!("");
}

//Function that calculates how many Ludic numbers are below 'max_num'
fn print_num_ludics_upto(x: &Vec<usize>, max_num: usize) {
    let mut num: i32 = 0;
    for i in 0..x.len() {
        if x[i] < max_num {
            num = num + 1;
        }
    }
    println!("{}", num);
}

//Function that prints Ludic numbers between to numbers
fn print_ludics_from_to(x: &Vec<usize>, from: usize, to: usize) {
    for i in from - 1..to - 1 {
        print!("{} ", x[i]);
    }
    println!("");
}

//Function that calculates triplets until certain Ludic number
fn print_tiples_until(x: &Vec<usize>, limit: usize) {
    let mut counter: usize = 0;

    while x[counter] < limit {
        let triplet2 = x[counter] + 2;
        let triplet3 = x[counter] + 6;

        let res_triplet2 = x.binary_search(&triplet2);
        let res_triplet3 = x.binary_search(&triplet3);

        if res_triplet2.is_ok() && res_triplet3.is_ok() {
            println!("{} {} {}", x[counter], triplet2, triplet3);
        }
        counter = counter + 1;
    }
}
