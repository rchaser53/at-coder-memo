#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [Chars;n]
  }
  
  let mut b_start = vec![];
  let mut a_end = vec![];
  let mut result = 0;
  for x in vals.iter() {
    if x[x.len()-1] == 'A' {
      a_end.push(x);
    }
    if x[0] == 'B' {
      b_start.push(x);
    }
    
    for i in 0..x.len()-1 {
      if x[i] == 'A' && x[i+1] == 'B' {
        result += 1;
      }
    }
  }
  
  if a_end.len() == b_start.len()
    && a_end == b_start
    && 0 < a_end.len() {
    result += a_end.len() - 1;
  } else {
    result += std::cmp::min(a_end.len(), b_start.len());
  }
  println!("{}", result);
}