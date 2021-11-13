#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: u128,
    mut p: u128,
  }
  
  let mut map:HashMap<u128, u128> = HashMap::new();
  let mut a = 2;
  let base = p;
  while a * a <= base {
    if p % a == 0 {
      let entry = map.entry(a).or_insert(0);
      *entry += 1;
      p /= a;
    } else {
      a += 1;
    }
  }

  if p != 1 {
    let entry = map.entry(p).or_insert(0);
    *entry += 1;
  }
      
  let mut result = 1;
  for (key, v) in map {
    if n <= v {
      for _ in 0..(v/n) {
        result *= key;
      }
    }
  }
  println!("{}", result);
}