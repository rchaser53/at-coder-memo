#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashMap, HashSet};

fn main() {
  input! {
    n: usize,
    blues: [String;n],
    m: usize,
    reds: [String;m]
  }
  
  let mut map: HashMap<String, isize> = HashMap::new();
  
  for v in blues {
    let entry = map.entry(v).or_insert(0);
    *entry += 1;
  }
  
  for v in reds {
    let entry = map.entry(v).or_insert(0);
    *entry -= 1;
  }
  
  let mut max = 0;
  for (_, value) in map {
    max = std::cmp::max(value, max);
  }
  println!("{}", max);
}