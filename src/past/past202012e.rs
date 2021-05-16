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

const MOD:usize = 1_000_000_007;

fn culc(base: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let h = base.len();
  let w = base[0].len();
  let mut result = vec![vec![0;h];w];
  for (i, row) in base.into_iter().enumerate() {
    for (r, c) in row.into_iter().enumerate() {
      result[r][h-i-1] = c;
    }
  }
  result
}

fn main() {
  input!{
    h: usize,
    w: usize,
    target: [Bytes;h],
    mut stamp: [Bytes;h]
  }

  for _ in 0..2 {
    while stamp.last().unwrap().iter().all(|&c| c == b'.') {
      stamp.pop();
    }
    for i in 0..stamp.len() {
      if !stamp[i].iter().all(|&c| c == b'.') {
        stamp.drain(..i);
        break
      }
    }
    stamp = culc(stamp);
  }
  
  for _ in 0..4 {
    let h2 = stamp.len();
    let w2 = stamp[0].len();
    if h2 <= h && w2 <= w {
      for i in 0..=h-h2 {
        'outer: for j in 0..=w-w2 {
          for (k, t) in stamp.iter().enumerate() {
            for (l, c) in t.iter().enumerate() {
              if *c == b'#' && target[i+k][j+l] == b'#' {
                continue 'outer;
              }
            }
          }
          println!("Yes");
          return
        }
      }
    }
    stamp = culc(stamp);
  }
  println!("No");
}