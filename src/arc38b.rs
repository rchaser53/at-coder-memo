#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use superslice::Ext;
 
const MOD:usize = 1_000_000_007;

fn is_win(
  rows: &Vec<Vec<char>>,
  r: usize,
  c: usize,
  memo: &mut Vec<Vec<Option<bool>>>
) -> bool {
  if let Some(v) = memo[r][c] {
    v
  } else if rows[r][c] == '#' {
    true
  } else {
    let v1 = !is_win(&rows, r+1, c, memo);
    let v2 = !is_win(&rows, r+1, c+1, memo);
    let v3 = !is_win(&rows, r, c+1, memo);
    let v = v1 || v2 || v3;
    memo[r][c] = Some(v);
    v
  }
}
 
fn main() {
  input!{
    h: usize,
    w: usize,
    mut rows: [Chars;h]
  }
  for i in 0..h {
    rows[i].push('#');
  }
  rows.push((0..w+1).map(|_| '#').collect::<Vec<char>>());
  let mut memo = vec![vec![None;w+2];h+2];
  if is_win(&rows, 0, 0, &mut memo) {
    println!("First");  
  } else {
    println!("Second");
  }
}