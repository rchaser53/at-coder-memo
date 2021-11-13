#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    c: usize,
    converters: [[usize;c];c],
    rows: [[Usize1;n];n],
  }
  
  if n == 1 {
    println!("0");
    return
  }

  let mut memo = vec![HashMap::new();3];
  for i in 0..n {
    for ii in 0..n {
      *memo[(i+ii+2) % 3].entry(rows[i][ii]).or_insert(0) += 1;
    }
  }

  let mut scores = vec![vec![(0, 1_000_000_000_000);c];3];
  for i in 0..3 {
    for to in 0..c {
      let mut temp = 0;
      for (from, num) in memo[i].iter() {
        let from = *from;
        temp += converters[from][to] * num;
      }
      scores[i][to] = (to, temp);
    }
    scores[i].sort_by(|a, b| a.1.cmp(&b.1));
  }

  let mut result = 1_000_000_000;
  for i in 0..3 {
    let (i1, v1) = scores[0][i];
    for ii in 0..3 {
      let (i2, v2) = scores[1][ii];
      for iii in 0..3 {
        let (i3, v3) = scores[2][iii];
        if i1 != i2 && i2 != i3 && i1 != i3 {
          result = std::cmp::min(result, v1 + v2 + v3);
        }
      }
    }
  }
  println!("{}", result);
}