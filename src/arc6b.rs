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

fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s = s.trim().to_string();
  let s = s.split(" ")
   .into_iter()
   .map(|v| v.parse::<usize>().unwrap())
   .collect::<Vec<usize>>();
  let n = s[0];
  let l = s[1];
    
  let mut rows = vec![];
  for _ in 0..l {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    rows.push(s.chars().collect::<Vec<char>>());  
  }
  
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s = s.to_string();
  let maru = s.chars().collect::<Vec<char>>();
  
  let num = 2*n-1;
  let mut memo = vec![vec![(200,200);n];l+1];
  for i in 0..l {
    if 1 < n && rows[i][1] == '-' {
      memo[i][0] = (1,i+1);
    } else {
      memo[i][0] = (0,i+1);
    }
    
    if 1 < n && rows[i][num-2] == '-' {
      memo[i][n-1] = (n-2,i+1);
    } else {
      memo[i][n-1] = (n-1,i+1);
    }
    
    for ii in 1..n-1 {
      if rows[i][2*ii-1] == '-' {
        memo[i][ii] = (ii-1, i+1);
      } else if rows[i][2*ii+1] == '-' {
        memo[i][ii] = (ii+1, i+1);
      } else {
        memo[i][ii] = (ii, i+1);
      }
    }
  }
  
  let goal = (100, 100);
  for i in 0..maru.len() {
    if maru[i] == 'o' {
      memo[l][i/2] = goal.clone();
      break
    }
  }

  for i in 0..n {
    let mut point = (i, 0);
    while point != (200,200) {
      point = memo[point.1][point.0];
      if &point == &goal {
        println!("{}", i+1);
        return
      }
    }
  }
}