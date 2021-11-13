#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    mut vals: [isize;n],
  }
  
  let mut memo: Vec<isize> = vec![0;n];
  memo[0] = vals[0];
  
  for i in 1..n {
    memo[i] = memo[i-1] + vals[i];
  }
  
  let mut map: HashMap<isize, usize> = HashMap::new();
  for i in 0..n {
    let entry = map.entry(memo[i]).or_insert(0);
    *entry += 1;
  }
  
  let mut count = 0;
  for (key, val) in map {
    count += c2(val);
    if key == 0 {
      count += val;
    }
  }
  println!("{}", count);
}

fn c2(a: usize) -> usize {
  a * (a - 1) / 2
}
