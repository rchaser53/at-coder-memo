#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    vals: [isize;n]
  }
  let inn = n as isize;
  let total = vals.iter().sum::<isize>();
  if total % inn != 0 {
    println!("-1");
    return
  }
  let average = vals.iter().sum::<isize>() / inn;
  let mut count = 0;
  let mut current = 0;
  for i in 0..n {
    let diff = average - vals[i];    
    current -= diff;
    
    if current != 0 {
      count += 1;
    }
  }
  println!("{}", count);
}
