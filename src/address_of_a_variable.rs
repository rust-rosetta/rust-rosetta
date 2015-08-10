/// http://rosettacode.org/wiki/Address_of_a_variable
#[cfg(not(test))]
fn main() {
    // It is not possible to change the memory address of an existing variable in Rust.

    // get the memory address of a variable
    let var: u32 = 1;
    let raw: *const u32 = &var;
    println!("address of var: {:?}", raw);

    // get the value at a certain memory address
    let address: usize = 0x7ffc8f303130;
    unsafe {
        let val = *(address as *const usize);
        println!("value at {}: {:?}", address, val);
    }

    // set the value at a certain memory address
    unsafe {
        *(address as *mut usize) = 1;
    }
}
