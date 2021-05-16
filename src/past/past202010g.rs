#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn helper(
  rows: &Vec<Vec<char>>,
  row: usize,
  column: usize,
  limit: usize,
  ri: usize,
  ci: usize,
) -> bool {
  let mut seen = HashSet::new();
  let mut stack = VecDeque::new();
  stack.push_back((ri, ci));
  seen.insert((ri, ci));

  while !stack.is_empty() {
    let (r, c) = stack.pop_front().unwrap();
    if 0 < r && rows[r-1][c] == '.' && !seen.contains(&(r-1, c)) {
      seen.insert((r-1, c));
      stack.push_back((r-1, c));
    }
    
    if 0 < c && rows[r][c-1] == '.' && !seen.contains(&(r, c-1)) {
      seen.insert((r, c-1));
      stack.push_back((r, c-1));
    }
    
    if c < column - 1 && rows[r][c+1] == '.' && !seen.contains(&(r, c+1)) {
      seen.insert((r, c+1));
      stack.push_back((r, c+1));
    }
    
    if r < row-1 && rows[r+1][c] == '.' && !seen.contains(&(r+1, c)) {
      seen.insert((r+1, c));
      stack.push_back((r+1, c));
    }
  }
  
  seen.len() == limit
}


fn main() {
  input!{
    n: usize,
    m: usize,
    rows: [Chars;n]
  }
  
  let mut count = 1;
  for i in 0..n {
    for ii in 0..m {
      if rows[i][ii] == '.' {
        count += 1;
      }
    }
  }
    
  let mut result = 0;
  for i in 0..n {
    for ii in 0..m {
      if rows[i][ii] == '#' && helper(&rows, n, m, count, i, ii) {
        result += 1;
      }
    }
  }
  println!("{}", result);
}