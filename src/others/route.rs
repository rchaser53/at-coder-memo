#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;
fn main() {
  input!{
    h: usize,
    w: usize,
    vals: [[usize;w];h]
  }
  
  let mut memo = vec![vec![0;w];h];
  let mut seen = HashSet::new();
  let mut stack = vec![];
  for i in 0..h {
    for ii in 0..w {
      if seen.contains(&(i, ii)) { continue };
      stack.push((i, ii));
      while !stack.is_empty() {
        let (r, c) = *stack.last().unwrap();
        let v = vals[r][c];
        let mut temp = 1;
        if 0 < r && v < vals[r-1][c] {
          if memo[r-1][c] == 0 {
            stack.push((r-1, c));
            continue
          } else {
            temp += memo[r-1][c];
            temp %= MOD;
          }
        }
        
        if r < h-1 && v < vals[r+1][c] {
          if memo[r+1][c] == 0 {
            stack.push((r+1, c));
            continue
          } else {
            temp += memo[r+1][c];
            temp %= MOD;
          }
        }
        
        if 0 < c && v < vals[r][c-1] {
          if memo[r][c-1] == 0 {
            stack.push((r, c-1));
            continue
          } else {
            temp += memo[r][c-1];
            temp %= MOD;
          }
        }
        
        if c < w-1 && v < vals[r][c+1] {
          if memo[r][c+1] == 0 {
            stack.push((r, c+1));
            continue
          } else {
            temp += memo[r][c+1];
            temp %= MOD;
          }
        }
        stack.pop();
        memo[r][c] = temp;
        seen.insert((r,c));
      }
    }
  }
  
  let mut result = 0;
  for i in 0..h {
    for ii in 0..w {
      result += memo[i][ii];
      result %= MOD;
    }
  }
  println!("{}", result);
}