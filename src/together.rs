#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [isize;n]
  }
  
  let mut map: HashMap<isize, usize> = HashMap::new();
  for v in vals {
    let entry = map.entry(v).or_insert(0);
    *entry += 1;

    let entry = map.entry(v-1).or_insert(0);
    *entry += 1;
    
    let entry = map.entry(v+1).or_insert(0);
    *entry += 1;
  }
  
  let mut max = 1;
  for (_, val) in map {
    max = std::cmp::max(max, val);
  }
  println!("{}", max);
}