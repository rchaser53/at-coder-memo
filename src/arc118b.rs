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
      k:usize,
      n:isize,
      m:isize,
      vals:[isize;k]
    }
    let mut result = vals
      .iter()
      .map(|v| (m as f64 * *v as f64 / n as f64) as isize)
      .collect::<Vec<isize>>();
    let mut cis = result
      .iter()
      .zip(vals.iter())
      .map(|(li,ai)| n * li - m * ai)
      .enumerate()
      .collect::<Vec<(usize,isize)>>();
    cis.sort_by_key(|a| a.1);
    let diff = m - result.iter().sum::<isize>();
    for i in 0..diff {
      result[cis[i as usize].0] += 1;
    }
    println!("{}", result
      .into_iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
      .join(" ")
    );
}
