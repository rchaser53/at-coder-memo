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
      s:Chars,
      t:usize
    }

    let mut result = vec![];
    for i in 0..s.len() {
      if i % t == 0 {
        result.push(s[i].to_string());
      }
    }

    println!("{}", result.into_iter().collect::<String>());
}
