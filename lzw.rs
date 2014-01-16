// Implements http://rosettacode.org/wiki/LZW_compression

use std::hashmap::HashMap;
use std::vec;
use std::str;

fn compress(original_str: &str) -> ~[int] {
   let original = original_str.as_bytes();
   let mut dict_size = 256;
   let mut dictionary = HashMap::new();
   
   for i in range(0, dict_size) {
      dictionary.insert(~[i as u8], i);
   }

   let mut result = ~[];
   let mut w:~[u8] = ~[];
   for &c in original.iter() {
      let mut wc = w.clone();
      wc.push(c);

      match dictionary.find(&wc) {
         Some(_) => w = wc,
         None => {
            result.push(*dictionary.get(&w));
            dictionary.insert(wc, dict_size);
            dict_size += 1;
            w = ~[c];
         }
      }
   }

   if w.len() > 0 {
      result.push(*dictionary.get(&w));
   }

   result
}

fn decompress(compressed: &[int]) -> ~str {
   let mut dict_size = 256;
   let mut dictionary = HashMap::new();
   
   for i in range(0, dict_size) {
      dictionary.insert(i, ~[i as u8]);
   }

   let mut w = ~[compressed[0] as u8];
   let compressed = compressed.slice(1, compressed.len());
   let mut result = w.clone();
   for &k in compressed.iter() {
      let entry = 
         match dictionary.find(&k) {
            Some(v) => v.clone(),
            None if k == dict_size => vec::append_one(w.clone(), w[0]),
            None => fail!("Invalid compressed string")
         };
      
      result = vec::append(result, entry);
      dictionary.insert(dict_size, vec::append_one(w, entry[0]));
      dict_size += 1;
      w = entry; 
   }

   str::from_utf8(result)
}

fn main() {
   let original = "TOBEORNOTTOBEORTOBEORNOT";

   let compressed = compress(original);
   println!("{:?}", compressed);

   let decompressed = decompress(compressed);
   println(decompressed);

   // Check if the decompressed string corresponds to the original string
   assert!(original == decompressed);
}
