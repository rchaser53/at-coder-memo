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
      n:usize,
      vals:[usize;n]
    }

    let mut count = 0;
    for i in 0..n {
      if i % 2 == 0 && vals[i] % 2 == 1 {
        count += 1;
      }
    }
    println!("{}", count);
}
