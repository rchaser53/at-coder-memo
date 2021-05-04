#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use std::cmp::Reverse;
 
fn main() {
  input!{
    n:usize,
    m:usize,
    q:usize,
    vals:[(Usize1,Usize1,usize);m],
    dict: [usize;q]
  }
  let mut g = vec![vec![];n];
  for (from, to, v) in vals {
    g[from].push((to, v));
    g[to].push((from, v));
  }
  let mut done = vec![false;n];
  done[0] = true;
  let mut heap = BinaryHeap::new();
  for &(u,c) in g[0].iter() {
    heap.push((Reverse(c), u));
  }
  let mut count = 1;
  for x in dict {
    let mut next = vec![];
    while let Some(&(Reverse(d),v)) = heap.peek() {
      if x < d { break }
      heap.pop();
      if done[v] { continue }
      done[v] = true;
      count += 1;
      for &(u, c) in g[v].iter() {
        next.push((Reverse(c), u));
      }
    }
    heap.extend(next.into_iter());
    println!("{}", count);
  }
}
