use proconio::input;
use std::collections::HashMap;
 
fn main() {
  input! {
    N: usize,
    arr: [String;N]
  }
 
  let mut map: HashMap<String, bool> = HashMap::new();
 
  for v in arr {
    map.insert(v, true);
  }
  
  println!("{}", map.keys().len());
}