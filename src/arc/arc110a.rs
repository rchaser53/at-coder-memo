#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn culc(mut n:usize) -> HashMap<usize, usize> {
  let mut map = HashMap::new();
  let mut i = 2;
  while i * i <= n {
    if n % i == 0 {
      *map.entry(i).or_insert(0) += 1;
      n /= i;
    } else {
      i += 1;
    }
  }
  if 1 < n {
    *map.entry(n).or_insert(0) += 1;
  }
  map
}

fn main() {
  input!{
    n: usize,
  }
  
  let mut temp = 1;
  for i in 2..=n {
    let map = culc(i);
    let mut should_continue = true;
    for (key, num) in map {
      for _ in 0..num {
        if temp % i != 0 {
          temp *= key;
        } else {
          should_continue = false;
          break
        }
      }
      
      if !should_continue {
        break
      }
    }
  }
  let result = temp+1;
  println!("{}", result);
}