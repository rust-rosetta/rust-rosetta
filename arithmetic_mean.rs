// Implements http://rosettacode.org/wiki/Averages/Arithmetic_mean

fn mean(list: &[f64]) -> f64 {
   let mut sum: f64 = 0.0;
   for &i in list.iter() {
      sum += i;
   }
   
   let list_len = list.len();
   if list_len == 0 {
      fail!("Expected a non-empty list")
   }
   else {
      sum / (list_len as f64)
   }
}

fn main() {
   let input = ~[3.0, 1.0, 4.0, 1.0, 5.0, 9.0];
   let result = mean(input);

   // 3.833333
   println!("{}", result);
}
