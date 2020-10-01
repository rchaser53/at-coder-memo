#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashMap,VecDeque};

fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [usize;n]
  }
  
  let mut map: HashMap<usize, usize> = HashMap::new();
  for v in vals {
    let entry = map.entry(v).or_insert(0);
    *entry += 1;
  }
  
  let mut keys: Vec<usize> = map.keys().map(|v| *v).collect();
  let kinds = keys.len();
  
  if kinds <= k {
    println!("0");
    return
  }
  
  keys.sort_by(|a, b| map.get(&a).unwrap().cmp(&map.get(&b).unwrap()));
  
  let mut total = 0;
  let diff = kinds - k;
  for i in 0..diff {
    total += *map.get(&keys[i]).unwrap();
  }
  
  println!("{}", total);
}
