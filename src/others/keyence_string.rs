#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    s: String
  }

  let keyence = String::from("keyence");
  for i in 0..7 {
    let val = format!(
      "{}{}",
      s.get(0..7-i).unwrap(),
      s.get(s.len()-i..s.len()).unwrap_or("")
    );
    if val == keyence {
      println!("YES");
      return
    }
  }
  println!("NO");
}