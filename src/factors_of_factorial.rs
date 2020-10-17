#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn helper(
  map: &mut HashMap<usize, usize>,
  limit:usize
) {
  let mut i = 2;
  let mut a = limit;
  while i * i <= limit {
    if a % i == 0 {
      let entry = map.entry(i).or_insert(0);
      *entry += 1;
      *entry %= MOD;
      a /= i;
    } else {
      i += 1;
    }
  }
  if a != 1 {
    let entry = map.entry(a).or_insert(0);
    *entry += 1;
    *entry %= MOD;
  }
}

const MOD:usize = 1_000_000_007;
fn main() {
  input! {
    n: usize,
  }
  
  let mut map = HashMap::new();  
  for i in 2..=n {
    helper(&mut map, i);
  }
    
  let mut count = 1;
  for (_, val) in map {
    count *= (val + 1);
    count %= MOD;
  }
  println!("{}", count);
}