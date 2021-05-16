#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    x: isize,
    y: isize,
    mut blocks:[(isize, isize);n]
  }
  
  let mut sx = 0;
  let mut lx = 0;
  let mut sy = 0;
  let mut ly = 0;
  
  blocks.push((x, y));
  for i in 0..=n {
    sx = std::cmp::min(sx, blocks[i].0);
    lx = std::cmp::max(lx, blocks[i].0);
    sy = std::cmp::min(sy, blocks[i].1);
    ly = std::cmp::max(ly, blocks[i].1);
  }
  blocks.pop();

  let mut bareers = HashSet::new();
  for i in 0..n {
    bareers.insert(blocks[i]);
  }
  
  let mut memo = HashMap::new();
  let mut stack = VecDeque::new();
  stack.push_back((0,0, 0));
  memo.insert((0,0),0);
  while !stack.is_empty() {
    let (x, y, v) = stack.pop_front().unwrap();
    if sy - 1 <= y {
      let mut entry = memo.entry((x, y-1)).or_insert(100_000);
      if v+1 < *entry && !bareers.contains(&(x, y-1)) {
        *entry = v+1;
        stack.push_back((x, y-1, v+1));
      }
    }
      
    if sx - 1 <= x {
      let mut entry = memo.entry((x-1, y)).or_insert(100_000);
      if v+1 < *entry && !bareers.contains(&(x-1, y)) {
        *entry = v+1;
        stack.push_back((x-1, y, v+1));
      }
    }
    if x <= lx + 1 {
      let mut entry = memo.entry((x+1, y)).or_insert(100_000);
      if v+1 < *entry && !bareers.contains(&(x+1, y)) {
        *entry = v+1;
        stack.push_back((x+1, y, v+1));
      }
    }
      
    if y <= ly + 1 {
      if sx - 1 <= x {
        let mut entry = memo.entry((x-1, y+1)).or_insert(100_000);
        if v+1 < *entry && !bareers.contains(&(x-1, y+1)) {
          *entry = v+1;
          stack.push_back((x-1, y+1, v+1));
        }
      }
      let mut entry = memo.entry((x, y+1)).or_insert(100_000);
      if v+1 < *entry && !bareers.contains(&(x, y+1)) {
        *entry = v+1;
        stack.push_back((x, y+1, v+1));
      }
      if x <= lx + 1 {
        let mut entry = memo.entry((x+1, y+1)).or_insert(100_000);
        if v+1 < *entry && !bareers.contains(&(x+1, y+1)) {
          *entry = v+1;
          stack.push_back((x+1, y+1, v+1));
        }
      } 
    }
  }

  if let Some(v) = memo.get(&(x, y)) {
    println!("{}", v);
  } else {
    println!("-1");
  }
} 
