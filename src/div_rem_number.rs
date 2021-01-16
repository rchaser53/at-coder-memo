#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: u128,
  }
 
  let mut result = 0;
  let mut i = 1;
  loop {
    let left = n - i;
    let v = left / i;
    if v <= i { break }
    if left % i == 0 {
      result += v;
    }
    i += 1;
  }
  
  println!("{}", result);
}