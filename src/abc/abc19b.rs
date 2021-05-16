#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    s: Chars
  }
  
  let mut last = s[0];
  let mut count = 1;
  let mut result = String::from("");
  
  for i in 1..s.len() {
    let c = s[i];
    if c == last {
      count += 1;
    } else {
      result = format!("{}{}{}", result, last.to_string(), count);
      last = c;
      count = 1;
    }
  }
  println!("{}{}{}", result, last.to_string(), count);
}
