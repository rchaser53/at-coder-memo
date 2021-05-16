#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize
  }
  
  let mut arr = vec![
    String::from("1"),
    String::from("2"),
    String::from("3"),
    String::from("4"),
    String::from("5"),
    String::from("6")
  ];
  let v = n % 30;
  for i in 0..v {
    arr.swap(i % 5, i % 5 + 1);
  }
  println!("{}", arr.into_iter().collect::<String>());
}