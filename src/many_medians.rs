#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::HashMap;

#[fastout]
fn main() {
  input! {
    n: usize,
    vals: [usize;n]
  }

  let mut vis = vals
  .into_iter()
  .enumerate()
  .map(|(i, v)| (i, v))
  .collect::<Vec<(usize, usize)>>();
  vis.sort_by(|b,a| {
    let v = b.1.cmp(&a.1);
    if v == Ordering::Equal {
      b.0.cmp(&a.0)
    } else {
      v
    }
  });
  
  let mut map:Vec<usize> = vec![0;n];
  for i in 0..n {
    map[vis[i].0] = i;
  }
  
  let mi = n / 2 - 1;
  for i in 0..n {
    let ti = map[i];
    if ti <= mi {
      println!("{}", vis[n/2].1);
    } else {
      println!("{}", vis[n/2 - 1].1);
    }
  }
}