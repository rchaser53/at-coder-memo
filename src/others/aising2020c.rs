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
    }

    let mut memo = vec![0;10usize.pow(5)];
    let limit = 100usize;
    for i in 1..=limit {
      for j in 1..=limit {
        for k in 1..=limit {
          let v = i.pow(2) + j.pow(2) + k.pow(2) + i * j + j * k + k * i;
          memo[v] += 1;
        }
      }
    }

    for i in 1..=n {
      println!("{}", memo[i]);
    }
}
