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
      q:usize,
      mut vals:[usize;n],
      queries:[(usize,usize,usize);q]
    }

    let mut base = 0;
    for (t, a, b) in queries {
      if t == 1 {
        let a = a - 1;
        let b = b - 1;
        vals.swap((a+n-base) % n, (b+n-base) % n);
      } else if t == 2 {
        base += 1;
      } else {
        let a = a - 1;
        println!("{}", vals[(a+n-base) % n]);
      }
      base %= n;
    }
}
