fn main() {    
   let mut current = 0i32;
   while (current * current)%1000000!=269696
   {current+=1;
   }
   println!("The smallest number whose square ends in 269696 is {}",current);
}
