fn main() {
    // Get the memory address of a variable
    let mut var = 1_i32;
    println!("address of var: {:p}", &var);

    // Get the value at a certain memory address
    let address: *mut i32 = &mut var;
    println!("value at {:p}: {:?}", address, var);

    // Set the value at a certain memory address
    unsafe {
        *(address as *mut i32) = 0;
        println!("value at {:p}: {:?}", address, var);
    }
}
