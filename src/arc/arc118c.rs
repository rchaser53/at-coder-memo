/* OUTPUT FILE */
#![allow(unused_imports)]
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn culc(
  set: &mut HashSet<usize>,
  mut base: usize,
  n: usize
) {
  let mut i = 2;
  while set.len() < n && base * i <= 10000 {
    let v = base * i;
    if !set.contains(&v) {
        set.insert(v);
    }
    i += 1;
  }
}

pub fn main(
) {
    input! {
      n:usize,
    }

    let mut set = HashSet::new();
    set.insert(6);
    set.insert(10);
    set.insert(15);

    culc(&mut set, 6, n);
    culc(&mut set, 10, n);
    culc(&mut set, 15, n);

    let result = set
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    println!("{}", result.join(" "));
}
