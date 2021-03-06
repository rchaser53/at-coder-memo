#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use num::Num;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    s: Chars
  }
  
  let mut result = vec![];
  for c in s {
    if b'0' <= c as u8 && c as u8 <= b'9' {
      result.push(c.to_string());
    }
  }
  println!("{}", result.into_iter().collect::<String>());
}