#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: isize,
    k: isize
  }
  let un = n as usize;
  let limit = 2*n as usize + 1;
  let mut memo: Vec<usize> = vec![0;limit];
  for i in 2..memo.len() {
    memo[i] = i-1;
    if un + 1 < i {
      memo[i] -= (i-un-1) * 2;
    }
  }
    
  let mut result = 0;
  for i in 2..=2*un {
    let ii = i as isize;
    if ii < k + 2 { continue }
    let ri = (ii - k) as usize;
    if limit <= ri { continue }
    result += memo[i] * memo[ri];
  }
  println!("{}", result);
}