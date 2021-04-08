#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    r:usize,
    c:usize,
    k:usize,
    vals:[Chars;r]
  }
  
  let mut memo = vec![vec![0;c];r];
  for i in 0..c {
    let mut count = 0;
    for ii in 0..r {
      count = if vals[ii][i] == 'o' { count + 1 } else { 0 };
      memo[ii][i] = count;
    }
  }
  for i in 0..c {
    let mut count = 0;
    for ii in (0..r).rev() {
      count = if vals[ii][i] == 'o' { count + 1 } else { 0 };
      memo[ii][i] = std::cmp::min(memo[ii][i], count);
    }
  }
  
  let mut count = 0;
  for i in k-1..=r-k {
    for ii in k-1..=c-k {
      if (0..k).all(|l| {
          k-l <= memo[i][ii-l]
          && k-l <= memo[i][ii+l] 
        }) {
        count += 1;
      }
    }
  }
  println!("{}", count);
}