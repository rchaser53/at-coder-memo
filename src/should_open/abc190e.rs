/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering::*;

fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize
) -> Vec<usize> {
  let mut score = vec![default_val;n];
  let mut pq = BinaryHeap::new();
  score[start] = 0;
  pq.push(std::cmp::Reverse((0, start)));
  while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
    if score[u] < w_u {
      continue
    }
    
    for &(v, w_v) in graph[u].iter() {
      let w = w_u + w_v;
      if w < score[v] {
        score[v] = w;
        pq.push(std::cmp::Reverse((w, v)));
      }
    }
  }
  score
}

fn main() {
  input! {
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1);m],
    k:usize,
    vals:[Usize1;k]
  }

  let mut g = vec![vec![];n];
  for (a, b) in edges {
    g[a].push((b,1));
    g[b].push((a,1));
  }

  let inf = 1_000_000_000_000_000usize;
  let mut memo = vec![vec![inf;k];k];
  for i in 0..k {
    memo[i][i] = 0;
    let ci = vals[i];
    let scores = dijkstra(n, inf, &g, ci);
    for j in 0..k {
      let ti = vals[j];
      memo[i][j] = scores[ti];
    }
  }

  let limit = 1 << k;

  let mut dp = vec![vec![inf;limit];k];
  for i in 0..k {
    dp[i][0] = 0;
  }

  // 現在まで寄ったところ
  for i in 0..limit {
    // 現在地点
    for j in 0..k {
      // 次
      for l in 0..k {
        // 既に来ている
        if i >> l & 1 == 1 { continue }
        let ni = i | (1 << j);
        dp[l][ni] = std::cmp::min(dp[l][ni], dp[j][i] + memo[j][l]);
      }
    }
  }

  let mut result = inf;
  for i in 0..k {
    result = std::cmp::min(result, dp[i][limit-1]);
  }

  if result == inf {
    println!("-1");
  } else {
    println!("{}", result + 1);
  }
}