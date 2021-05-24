/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
    k:usize,
    vals:[(usize,usize);n]
  }

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    for i in 0..n {
      let v = vals[i].1;
      heap.push((v,i));
    }

    let mut result = 0usize;
    for _ in 0..k {
      let (v, ti) = heap.pop().unwrap();
      result += v;
      if seen.contains(&ti) { continue }
      seen.insert(ti);
      heap.push((vals[ti].0 - vals[ti].1, ti));
    }
    println!("{}", result);
}
