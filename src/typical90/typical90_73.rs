/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn dfs(
  edges: &Vec<Vec<usize>>,
  vals: &Vec<char>,
  dp: &mut Vec<Vec<usize>>,
  ci:usize,
  last:usize
) {
  let mut val1 = 1;
  let mut val2 = 1;
  for &i in &edges[ci] {
    if i == last { continue }
    dfs(edges, vals, dp, i, ci);
    if vals[ci] == 'a' {
      val1 *= dp[i][0] + dp[i][2];
      val2 *= dp[i][0] + dp[i][1] + 2 * dp[i][2];
    }

    if vals[ci] == 'b' {
      val1 *= dp[i][1] + dp[i][2];
      val2 *= dp[i][0] + dp[i][1] + 2 * dp[i][2];
    }
    val1 %= MOD;
    val2 %= MOD;
  }

  if vals[ci] == 'a' {
    dp[ci][0] = val1;
    dp[ci][2] = (val2 - val1 + MOD) % MOD;
  }

  if vals[ci] == 'b' {
    dp[ci][1] = val1;
    dp[ci][2] = (val2 - val1 + MOD) % MOD;
  }
}

pub fn main(
) {
  input! {
    n:usize,
    vals:[char;n],
    edges:[(Usize1,Usize1);n-1]
  }

  let mut g = vec![vec![];n];
  for (l, r) in edges {
    g[l].push(r);
    g[r].push(l);
  }

  let inf = 1_000_000_000_000;
  let mut dp = vec![vec![0;3];n];

  dfs(&g, &vals, &mut dp, 0, inf);
  println!("{}", dp[0][2]);
}
