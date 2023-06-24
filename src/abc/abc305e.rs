/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    k:usize,
    ab:[(Usize1,Usize1);m],
    ph:[(Usize1,isize);k]
  }

  let mut g = vec![vec![];n];
  let mut seen = vec![false;n];
  let mut memo = vec![-1;n];

  for (a,b) in ab {
    g[a].push(b);
    g[b].push(a);
  }

  let mut heap = BinaryHeap::new();
  for (p, h) in ph {
    heap.push((h,p));
    memo[p] = memo[p].max(h);
    seen[p] = true;
  }

  while let Some((h, p)) = heap.pop() {
    seen[p] = true;
    let nh = h - 1;
    for &ni in &g[p] {
      if memo[ni] < nh {
        memo[ni] = nh;
        seen[ni] = true;
        heap.push((nh, ni));
      }
    }
  }

  let mut result = vec![];
  for i in 0..n {
    if seen[i] {
      result.push((i+1).to_string());
    }
  }

  println!("{}", result.len());
  if !result.is_empty() {
    println!("{}", result.join(" "));
  }  
}