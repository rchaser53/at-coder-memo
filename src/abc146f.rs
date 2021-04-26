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
use superslice::Ext;

const MOD:usize = 998244353;
const MAX:usize = 200_000;

#[fastout]
fn main() {
  input!{
    n:usize,
    m:usize,
    mut s:Chars
  }
  let inf = 1_000_000_000;
  let mut dp = vec![inf;n+1];
  dp[n] = 0;
  
  let mut ci = n;
  while 0 < ci {
    let last = ci;
    if ci <= m {
      dp[0] = std::cmp::min(dp[0], dp[ci]+1);
      break
    }
    
    for j in (1..=m).rev() {
      if ci < j { break }
      let next = ci - j;
      if s[next] == '1' { continue }
      dp[next] = std::cmp::min(dp[next], dp[ci]+1);
      ci = next;
      break
    }
    
    if ci == last {
      println!("-1");
      return
    }
  }
  
  if inf <= dp[0] {
    println!("-1");
  } else {
    let mut i = dp[0];
    let mut result = vec![];
    for j in 0..=n {
      if i == dp[j] {
        i -=1;
        result.push(j);
      }
    }
    
    let base = result.clone();
    for i in 1..result.len() {
      result[i] = base[i] - base[i-1];
    }
    result.remove(0);
        
    println!("{}", result
      .into_iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
      .join(" ")
    );
  }
}