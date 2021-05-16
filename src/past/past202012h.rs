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

#[fastout]
fn main() {
  input!{
    h: usize,
    w: usize,
    r: Usize1,
    c: Usize1,
    rows: [Chars;h]
  }
  
  let mut result = vec![vec!['a';w];h];
  let mut stack = VecDeque::new();
  result[r][c] = 'o';
  stack.push_back((r, c));
  
  while !stack.is_empty() {
    let (r, c) = stack.pop_front().unwrap();
    
    if 0 < r && result[r-1][c] == 'a' && rows[r-1][c] != '#' &&
      (rows[r-1][c] == 'v' || rows[r-1][c] == '.')
    {
      stack.push_back((r-1,c));
      result[r-1][c] = 'o';
    }

    if r < h-1 && result[r+1][c] == 'a' && rows[r+1][c] != '#' &&
      (rows[r+1][c] == '^' || rows[r+1][c] == '.')
    {
      stack.push_back((r+1,c));
      result[r+1][c] = 'o';
    }

    if 0 < c && result[r][c-1] == 'a' && rows[r][c-1] != '#' &&
      (rows[r][c-1] == '>' || rows[r][c-1] == '.')
    {
      stack.push_back((r,c-1));
      result[r][c-1] = 'o';
    }

    if c < w-1 && result[r][c+1] == 'a' && rows[r][c+1] != '#' &&
      (rows[r][c+1] == '<' || rows[r][c+1] == '.')
    {
      stack.push_back((r,c+1));
      result[r][c+1] = 'o';
    }
  }
  
  for i in 0..h {
    let mut row = result[i].clone();
    for ii in 0..w {
      if rows[i][ii] == '#' {
        row[ii] = '#';
      } else if row[ii] == 'a' {
        row[ii] = 'x';
      }
    }
    println!("{}", row.into_iter().map(|v| v.to_string()).collect::<String>());
  }  
}