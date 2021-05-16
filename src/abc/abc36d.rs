/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD: usize = 1_000_000_007;

fn dfs(g: &Vec<Vec<usize>>, memo: &mut Vec<(usize, usize)>, ci: usize, last: usize) {
    for &ni in &g[ci] {
        if ni == last {
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

    // 0: black 1: white
    let mut dp = vec![(1, 1); n];
    let mut g = vec![vec![]; n];
    for (from, to) in vals {
        g[from].push(to);
        g[to].push(from);
    }

    dfs(&g, &mut dp, 0, 1_000_000_000);
    println!("{}", (dp[0].0 + dp[0].1) % MOD);
}
