#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::HashMap;
 
fn main() {
  input!{
    n: usize,
    vals: [usize;n] 
  }
  
  let mut map = HashMap::new();
  for v in vals {
    let entry = map.entry(v).or_insert(false);
    *entry = !(*entry);
  }
  
  let mut count = 0;
  for (_, v) in map {
    if v { count += 1; }
  }
  println!("{}", count);
}
