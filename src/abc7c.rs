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
    r: usize,
    c: usize,
    sy: Usize1,
    sx: Usize1,
    gy: Usize1,
    gx: Usize1,
    rows:[Chars;r]
  }
  
  let mut memo = vec![vec![1_000_000;c];r];
  let mut stack = VecDeque::new();
  memo[sy][sx] = 0;
  stack.push_back((sx, sy));
  while !stack.is_empty() {
    let (x, y) = stack.pop_front().unwrap();
    if 0 < x
      && rows[y][x-1] == '.'
      && memo[y][x]+1 < memo[y][x-1] {
      memo[y][x-1] = memo[y][x]+1;
      stack.push_back((x-1,y));
    }

    if x < c
      && rows[y][x+1] == '.'
      && memo[y][x]+1 < memo[y][x+1] {
      memo[y][x+1] = memo[y][x]+1;
      stack.push_back((x+1,y));
    }

    if 0 < y
      && rows[y-1][x] == '.'
      && memo[y][x]+1 < memo[y-1][x] {
      memo[y-1][x] = memo[y][x]+1;
      stack.push_back((x,y-1));
    }
   
    if y < r
      && rows[y+1][x] == '.'
      && memo[y][x]+1 < memo[y+1][x] {
      memo[y+1][x] = memo[y][x]+1;
      stack.push_back((x,y+1));
    }
  }
  
  println!("{}", memo[gy][gx]);
}