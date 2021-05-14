/* OUTPUT FILE */
#![allow(unused_imports)]
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

pub fn main(
) {
    input! {
      a:usize,
      b:usize,
      c:usize,
      x:usize,
      y:usize
    }

    let large = 2 * std::cmp::max(x,y);
    let mut min = 1_000_000_000_000usize;
    for i in 0..=large {
      if i % 2 == 1 { continue }
      let cnn = i / 2;
      let mut temp = i * c;

      if cnn <= x {
        temp += (x - cnn) * a;
      }
      if cnn <= y {
        temp += (y - cnn) * b;
      }
      min = std::cmp::min(min, temp);
    }
    println!("{}", min);
}
