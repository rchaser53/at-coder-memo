#![allow(unused_imports)]
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

const MOD:usize = 1_000_000_007;
pub fn main(
) {
    input! {
      k:usize,
    }

    if k % 9 != 0 {
      println!("0");
    } else {
      let mut memo = vec![0;k+1];
      memo[0] = 1;
      for i in 1..=k {
        for j in 1..=9 {
          if i < j { break }
          memo[i] += memo[i-j];
          memo[i] %= MOD;
        }
      }
      println!("{}", memo[k]);
    }
}
