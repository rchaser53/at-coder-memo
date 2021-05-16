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

fn main() {
  input!{
    n:usize,
    m:usize,
    edge:[(Usize1,Usize1);m],
    s:Usize1,
    k:usize,
    mut t:[Usize1;k]
  }
  
  let mut memo = vec![vec![];n];
  for (a, b) in edge {
    memo[a].push(b);
    memo[b].push(a);
  }
  
  let inf = 1usize << 60;
  let mut dist = vec![vec![inf;n];k+1];
  t.push(s);
  let len = k + 1;
  for i in 0..len {
    let mut q = VecDeque::new();
    q.push_back((t[i], 0));
    dist[i][t[i]] = 0;
    while let Some((ti, cost)) = q.pop_front() {
      if dist[i][ti] < cost {
        continue;
      }
      let n_cost = cost + 1;
      for &next in memo[ti].iter() {
        if dist[i][next] <= n_cost {
          continue;
        }
        dist[i][next] = n_cost;
        q.push_back((next, n_cost));
      }
    }
  }
  
  let mut dp = vec![vec![inf;1<<len];len];
  dp[len-1][1 << (len-1)] = 0;
  for bit in 1..(1<<len) {
    for i in 0..len {
      if ((bit >> i) & 1) == 0 { continue }
      let b = bit & (!(1 << i));
      for j in 0..len {
        if ((b >> j) & 1) == 0 { continue }
        let cost = dp[j][b] + dist[j][t[i]];
        dp[i][bit] = std::cmp::min(dp[i][bit], cost);
      }
    }
  }
  let mut result = inf;
  for i in 0..len {
    result = std::cmp::min(dp[i][(1<<len)-1], result);
  }
  println!("{}", result);
}


