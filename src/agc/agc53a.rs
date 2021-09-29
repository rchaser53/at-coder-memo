use proconio::input;
use proconio::marker::*;
use std::collections::*;
use itertools::Itertools;
fn main() {
  input!{
    n:usize,
    s:Chars,
    vals:[usize;n+1]
  }

  let k = (0..n).map(|i| vals[i].max(vals[i+1]) - vals[i].min(vals[i+1])).min().unwrap();
  let mut result = vec![vec![0;n+1];k];
  for i in 0..=n {
    for j in 0..k {
      result[j][i] = (vals[i] + j) / k;
    }
  }

  println!("{}", k);
  for i in 0..k {
    println!("{}", result[i].iter().join(" "));
  }
}