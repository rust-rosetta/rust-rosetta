// Implements http://rosettacode.org/wiki/LZW_compression

extern crate collections;

use collections::hashmap::HashMap;
use std::str;

fn compress(original_str: &str) -> Vec<int> {
   let original = original_str.as_bytes();
   let mut dict_size = 256;
   let mut dictionary = HashMap::new();
   
   for i in range(0, dict_size) {
      dictionary.insert(vec!(i as u8), i);
   }

   let mut result = vec!();
   let mut w = vec!();
   for &c in original.iter() {
      let mut wc = w.clone();
      wc.push(c);

      match dictionary.find(&wc) {
         Some(_) => w = wc,
         None => {
            result.push(*dictionary.get(&w));
            dictionary.insert(wc, dict_size);
            dict_size += 1;
            w = vec!(c);
         }
      }
   }

   if w.len() > 0 {
      result.push(*dictionary.get(&w));
   }

   result
}

fn decompress(compressed: &Vec<int>) -> ~str {
   let mut dict_size = 256;
   let mut dictionary = HashMap::new();
   
   for i in range(0, dict_size) {
      dictionary.insert(i, vec!(i as u8));
   }

   let mut w = vec!(compressed.get(0).clone() as u8);
   let compressed = compressed.slice(1, compressed.len());
   let mut result = w.clone();
   for &k in compressed.iter() {
      let entry = 
         match dictionary.find(&k) {
            Some(v) => v.clone(),
            None if k == dict_size => { let mut new = w.clone(); new.push(w.get(0).clone()); new }
            None => fail!("Invalid compressed string")
         };
      
      result.extend(entry.iter().map(|&x| x.clone()));
      w.push(entry.get(0).clone());
      dictionary.insert(dict_size, w);
      dict_size += 1;
      w = entry; 
   }

   str::from_utf8(result.as_slice()).unwrap().to_owned()
}

fn main() {
   let original = "TOBEORNOTTOBEORTOBEORNOT";

   let compressed = compress(original);
   println!("{:?}", compressed);

   let decompressed = decompress(&compressed);
   println!("{:s}", decompressed);

   // Check if the decompressed string corresponds to the original string
   assert!(original == decompressed);
}
