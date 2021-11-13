use proconio::input;
use std::collections::HashMap;
use std::iter::Iterator;
use std::iter::FromIterator;
 
fn main() {
  input! {
    n: usize,
    mut ss: [String;n]
  }
  
  let mut map: HashMap<String, usize> = HashMap::new();
  for s in ss {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    let sorted = String::from_iter(chars);
    let mut a = map.entry(sorted).or_insert(0);
    *a += 1;
  }
  
  let mut result = 0;
  for (_, v) in map {
    result += (v * (v-1)) / 2;
  }
  
  println!("{}", result);
}