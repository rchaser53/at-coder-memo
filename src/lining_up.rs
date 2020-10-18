#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  let mut map = HashMap::new();
  if n % 2 == 0 {
    for i in 0..n {
      if vals[i] % 2 != 1 {
        println!("0");
        return
      }
      let entry = map.entry(vals[i]).or_insert(0);
      *entry += 1;
    }
    for (_, val) in map {
      if val != 2 {
        println!("0");
        return
      }
    }
  } else {
    for i in 0..n {
      if vals[i] % 2 != 0 {
        println!("0");
        return
      }
      let entry = map.entry(vals[i]).or_insert(0);
      *entry += 1;
    }
    for (key, val) in map {
      if key == 0 && val == 1 {
        continue;
      }
      
      if val != 2 {
        println!("0");
        return
      }
    }
  };
  
  let mut count = 1;
  let b = n / 2;
  for _ in 0..b {
    count *= 2;
    count %= MOD;
  }
  println!("{}", count);
}