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
    exps: [isize;n],
    scores: [isize;n]
  }
  
  if exps.iter().sum::<isize>() < scores.iter().sum::<isize>() {
    println!("-1");
    return
  }
  
  let mut uppers = vec![];
  let mut belows = vec![];
  for i in 0..n {
    let v = exps[i] - scores[i];
    if v < 0 {
      belows.push((i,v));
    } else {
      uppers.push((i,v));
    }
  }
  uppers.sort_by(|a,b| a.1.cmp(&b.1));
  belows.sort_by(|a,b| a.1.cmp(&b.1));
  let mut count = belows.len();
  let mut temp = 0;
  while !belows.is_empty() {
    temp += belows.pop().unwrap().1;
    while temp < 0 {
      temp += uppers.pop().unwrap().1;
      count += 1;
    }
  }
  println!("{}", count);
}