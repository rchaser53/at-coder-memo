#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
 
#[fastout]
fn main() {
  input!{
    s:Chars
  }
  
  let mut count = 0;
  for i in 0..s.len() {
    if i % 4 == 1 {
      count += 1;
      if s[i] == 'o' {
        println!("{}", count);
        return
      }
    }
  }
  println!("none");
}