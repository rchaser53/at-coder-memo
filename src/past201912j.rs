#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn culc(
  dp: &mut Vec<Vec<usize>>,
  rows: &Vec<Vec<usize>>,
  h: usize,
  w: usize,
  sr: usize,
  sc: usize
) {  
  let mut stack = VecDeque::new();
  stack.push_back((sr, sc));
  while let Some((r, c)) = stack.pop_front() {    
    if 0 < r && dp[r][c] + rows[r-1][c] < dp[r-1][c] {
      dp[r-1][c] = dp[r][c] + rows[r-1][c];      
      stack.push_back((r-1, c));
    }
    
    if r < h-1 && dp[r][c] + rows[r+1][c] < dp[r+1][c] {
      dp[r+1][c] = dp[r][c] + rows[r+1][c];
      stack.push_back((r+1, c));
    }
    
    if 0 < c && dp[r][c] + rows[r][c-1] < dp[r][c-1] {
      dp[r][c-1] = dp[r][c] + rows[r][c-1];
      stack.push_back((r, c-1));
    }
    
    if c < w-1 && dp[r][c] + rows[r][c+1] < dp[r][c+1] {
      dp[r][c+1] = dp[r][c] + rows[r][c+1];
      stack.push_back((r, c+1));
    }
  }
}

fn main() {
  input!{
    h: usize,
    w: usize,
    rows: [[usize;w];h]
  }
   
  let mut min = 5_000_000;
  let mut dp1 = vec![vec![5_000_000;w];h];
  dp1[h-1][0] = 0;
  let mut dp2 = vec![vec![5_000_000;w];h];
  dp2[h-1][w-1] = 0;
  let mut dp3 = vec![vec![5_000_000;w];h];
  dp3[0][w-1] = 0;
  culc(&mut dp1, &rows, h, w, h-1, 0);
  culc(&mut dp2, &rows, h, w, h-1, w-1);
  culc(&mut dp3, &rows, h, w, 0, w-1);
  
  for i in 0..h {
    for ii in 0..w {
      min = std::cmp::min(min, 
        dp1[i][ii] + dp2[i][ii] + dp3[i][ii] - rows[i][ii] * 2
      );
    }
  }

  println!("{}", min);
}