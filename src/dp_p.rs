// 問題文を言い換えているだけでABC36D 迷路と構成が全く同じと思われる
// https://atcoder.jp/contests/abc036/tasks/abc036_d
/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

const MOD:usize = 1_000_000_007;

pub fn dfs(
  g: &Vec<Vec<usize>>,
  memo: &mut Vec<(usize,usize)>,
  ci: usize,
  last: usize
) {
    for &ni in &g[ci] {
        if last == ni {
            continue;
        }
        dfs(g, memo, ni, ci);
        memo[ci].0 *= memo[ni].1;
        memo[ci].1 *= memo[ni].0 + memo[ni].1;
        memo[ci].0 %= MOD;
        memo[ci].1 %= MOD;
    }
}

pub fn main(
) {
    input! {
      n:usize,
      vals:[(Usize1,Usize1);n-1]
    }

    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        let (from, to) = vals[i];
        g[from].push(to);
        g[to].push(from);
    }

    let mut memo = vec![(1,1); n];
    dfs(&g, &mut memo, 0, 1_000_000_000);
    println!("{}", (memo[0].0 + memo[0].1) % MOD);
}
