/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
  }
  let mut g = vec![vec![];n];
  for i in 0..n {
    for j in 0..10 {
      let ni = (i+j*n) / 10;
      let cost = (i+j*n) % 10;
      g[i].push((ni, cost));
    }
  }

  let mut memo = vec![usize::max_value();n];
  let mut heap = BinaryHeap::new();
  heap.push((Reverse(0), 0));
  while let Some((Reverse(dx), x)) = heap.pop() {
    if memo[x] < dx {
      continue
    }
    for &(y, w) in &g[x] {
      let dy = dx + w;
      if dy < memo[y] {
        heap.push((Reverse(dy), y));
        memo[y] = dy;
      }
    }
  }

  let result = (1..10.min(n)).map(|i| i+memo[i]).min().unwrap();
  println!("{}", result);
}