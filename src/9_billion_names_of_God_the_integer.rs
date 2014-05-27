//Implements http://rosettacode.org/wiki/9_billion_names_of_God_the_integer

extern crate num;

use num::bigint::BigUint;
use std::from_str::FromStr;
use std::string::String;
use std::cmp::min;



 fn cumu<'a>(num: uint, cache: &'a mut Vec<Vec<BigUint>>) -> &'a Vec<BigUint> {
	
    let mut len =0;
    for num in cache.iter()
    {
    	len = len+1;
    }

    for l in range(len, num+1)
    {
	    let initial_value:BigUint = FromStr::from_str("0").unwrap();
	    let mut r: Vec<BigUint> = vec!(initial_value);
		
	    for x in range(1, l+1)
	    {
		 let y = r.get(x -1).clone();
		 let z = cache.get(l-x).get(min(x, l-x)).clone();
		 let w = y+z;
		
		 r.push(w)
                
	    }

	    cache.push(r);

    }

    cache.get(num)
}

// Returns a line
#[cfg(not(test))]
fn row(num: uint,  cache: &mut Vec<Vec<BigUint>>) -> String {
	 
	let r = cumu(num,cache);
	let mut returned_string = String::new();
	for i in range(0,num)
	{
		let i = *r.get(i+1) - *r.get (i);
		let z = i.to_str();
		let y = z.as_slice();
		returned_string.push_str(y);
		returned_string.push_str(", ");
	}
	returned_string
	
	 


}

#[cfg(not(test))]
fn main()
{

	    let mut cache: Vec<Vec<BigUint>> = Vec::new();

	    let initial_value:BigUint = FromStr::from_str("1").unwrap();

	    let initial_vector : Vec<BigUint> = vec!(initial_value);
	    cache.push(initial_vector);
	   


	    println!("rows");
	    for n in range(1, 11)
	    {	
			let x = n as uint;
			println!("{}: {}", n, row(x,&mut cache));
	    }

	    println!("sums");

	    let x: Vec<uint> = vec!(23, 123, 1234, 12345);
	    for y in x.iter()
	    {
			let z = cumu(*y,&mut cache);
			let w = z.last();
			println!("{}: {}", y, w.unwrap());
			

	   }
		

	    
           

}


#[test]
fn test_cumu()
{

	    let mut cache: Vec<Vec<BigUint>> = Vec::new();

	    let initial_value:BigUint = FromStr::from_str("1").unwrap();

	    let initial_vector : Vec<BigUint> = vec!(initial_value);
	    cache.push(initial_vector);


	    let a: Vec<uint> = vec!(23, 123, 1234);
	    let b: Vec<BigUint> = vec!(FromStr::from_str("1255").unwrap(), FromStr::from_str("2552338241").unwrap(), FromStr::from_str("156978797223733228787865722354959930").unwrap());
	    let mut n=0;
	    for y in a.iter()
	    {
			let z = cumu(*y,&mut cache);
			let w = z.last().unwrap();
			assert!(w == b.get(n));
			n= n+1;
			

	   }
}




