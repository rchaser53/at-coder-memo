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
use std::cmp::*;
use superslice::Ext;

const MOD:usize = 1_000_000_007;

fn is_success(
  rows: &mut Vec<Vec<char>>,
  mut count: usize,
  r: usize,
  c: usize,
) -> bool {
  let mut set = HashSet::new();  
  let mut dirty = false;
  if rows[r][c] == 'x' {
    rows[r][c] = 'o';
    dirty = true;
    count += 1;
  }

  let mut stack = vec![(r,c)];
  while !stack.is_empty() {
    let (rr, cc) = stack.pop().unwrap();
    if set.contains(&(rr, cc)) { continue }
    set.insert((rr, cc));
    
    if 0 < rr && rows[rr-1][cc] == 'o' {
      stack.push((rr-1, cc));
    }
    
    if rr < 9 && rows[rr+1][cc] == 'o' {
      stack.push((rr+1, cc));
    }
    
    if 0 < cc && rows[rr][cc-1] == 'o' {
      stack.push((rr, cc-1));
    }
    
    if cc < 9 && rows[rr][cc+1] == 'o' {
      stack.push((rr, cc+1));
    }
  }
  
  if dirty {
    rows[r][c] = 'x';  
  }
  set.len() == count
}

#[fastout]
fn main() {
  input!{
    mut rows: [Chars;10]
  }
  
  let mut count = 0;
  for i in 0..10 {
    for ii in 0..10 {
      if rows[i][ii] == 'o' {
        count += 1;
      }
    }
  }
  
  for i in 0..10 {
    for ii in 0..10 {
      if is_success(&mut rows, count, i, ii) {
        println!("YES");
        return
      }
    }
  }
  println!("NO"); 
}
