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
  memo: &mut Vec<Vec<Option<bool>>>,
  r: usize,
  c: usize
) -> bool {
  if let Some(v) = memo[r][c] {
    v
  } else if rows[r][c] == '#' {
    true
  } else {
    let v1 = !is_win(rows, memo, r+1, c);
    let v2 = !is_win(rows, memo, r+1, c+1);
    let v3 = !is_win(rows, memo, r, c+1);
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
  rows.push(vec!['#';w+1]);
  
  let mut memo = vec![vec![None;w+1];h+1];
  if is_win(&rows, &mut memo, 0, 0) {
    println!("First");
  } else {
    println!("Second");
  } 
}