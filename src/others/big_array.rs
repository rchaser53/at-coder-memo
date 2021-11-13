#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashSet, HashMap, VecDeque};
 
fn main() {
  input!{
    n: usize,
    mut k: usize,
    vals: [(usize, usize);n]
  }
  let mut map: HashMap<usize, usize> = HashMap::new();
  
  for (val, num) in vals {
    let entry = map.entry(val).or_insert(0);
    *entry += num;
  }
  
  let mut keys:Vec<&usize> = map.keys().collect();
  keys.sort();
  
  for key in keys {
    let v = map.get(key).unwrap();
    if &k <= v {
      println!("{}", key);
      return
    }
    k -= v;
  }
}