/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1);m]
  }
  let mut g = vec![vec![];n];
  for (a,b) in edges {
    g[a].push(b + 1);
    g[b].push(a + 1);
  }
  for i in 0..n {
    g[i].sort();

    println!("{} {}", g[i].len(), g[i].iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
  }
}