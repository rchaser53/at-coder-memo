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
  
  if s[3] != '-' {
    println!("No");
    return
  }
  
  for i in 0..s.len() {
    if i == 3  {
      continue
    }
    
    if s[i] < '0' || '9' < s[i] {
      println!("No");
      return
    }
  }
  println!("Yes"); 
}