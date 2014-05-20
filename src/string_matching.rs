// Implements http://rosettacode.org/wiki/String_matching

fn matchString(container: &str, target: &str) -> (bool,bool,bool) {
  let starts = container.starts_with(target);
  let ends = container.ends_with(target);
  (starts, starts||ends||container.contains(target), ends)
}

#[cfg(not(test))]
fn printInfo(container: &str, target: &str) {
  println!("Matching \"{}\" in the string \"{}\"", target, container);
  let (starts,contains,ends) = matchString(container,target);
  if starts {
    println!("\"{}\" starts with \"{}\"", container, target);
  }
  if contains {
    println!("\"{}\" contains \"{}\"", container, target);
  }
  if ends {
    println!("\"{}\" ends with \"{}\"", container, target);
  }
}

#[cfg(not(test))]
fn main() {
  printInfo("abcd","ab");
  printInfo("abcd","bc");
  printInfo("abcd","cd");
}


#[test]
fn testMatchString() {
  assert_eq!( matchString("abcd", "ab"), (true,true,false) );
  assert_eq!( matchString("abcd", "ba"), (false,false,false) );
  assert_eq!( matchString("abcd", "bc"), (false,true,false) );
  assert_eq!( matchString("abcd", "cd"), (false,true,true) );
}
