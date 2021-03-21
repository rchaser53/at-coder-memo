#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use superslice::*;
use std::collections::*;
use std::cmp::*;

const MOD:usize = 1_000_000_007;

// true:  Sheep
// false: Wolf
fn helper(
  s: &Vec<char>,
  start: (bool, bool)
) -> Option<String> {
  let n = s.len();
  let mut dp = vec![false;n+1];
  dp[0] = start.0;
  dp[1] = start.1;
  let last = if s[0] == 'o' {
    if dp[0] {
      dp[1]
    } else {
      !dp[1]
    }
  } else {
    if dp[0] {
      !dp[1]
    } else {
      dp[1]
    }
  };
  
  for i in 1..n {
    if s[i] == 'o' {
      if dp[i] {
        dp[i+1] = dp[i-1];
      } else {
        dp[i+1] = !dp[i-1];
      }
    } else {
      if dp[i] {
        dp[i+1] = !dp[i-1];
      } else {
        dp[i+1] = dp[i-1];
      }
    }
  }
  if dp[0] == dp[n] && last == dp[n-1] {
    dp.remove(n);
    let result = dp.into_iter()
      .map(|v| if v {
        "S".to_string()
        } else {
        "W".to_string()
      })
      .collect::<String>();
    Some(result)
  } else {
    None
  }
}

fn main() {
  input!{
    n:usize,
    s:Chars
  }
  
  let inputs = vec![(true, true), (true, false), (false, true), (false, false)];
  for (a, b) in inputs {
    if let Some(v) = helper(&s, (a,b)) {
      println!("{}", v);
      return
    }
  }
  println!("-1");
}