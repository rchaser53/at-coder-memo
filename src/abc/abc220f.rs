use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn dfs(tree: &Vec<Vec<usize>>, memo: &mut Vec<(usize,usize)>, ci:usize, last:usize) {
  for &ni in &tree[ci] {
    if ni == last { continue }
    dfs(tree, memo, ni, ci);
    memo[ci].0 += memo[ni].0 + memo[ni].1;
    memo[ci].1 += memo[ni].1;
  }
}

fn dfs2(
  tree: &Vec<Vec<usize>>,
  memo: &Vec<(usize,usize)>,
  result: &mut Vec<usize>,
  ci:usize,
  last:usize,
  children:usize,
  total:usize
) {
  result[ci] = total + children;
  for &ni in &tree[ci] {
    if ni == last { continue }
    let next_children = children + memo[ci].1 - memo[ni].1;
    dfs2(tree, memo, result, ni, ci, next_children, result[ci]-memo[ni].1);
  }
}

fn main() {
  input!{
    n:usize,
    edges:[(Usize1,Usize1);n-1]
  }

  let mut g = vec![vec![];n];
  for (f, t) in edges {
    g[f].push(t);
    g[t].push(f);
  }

  // 距離の合計, nodeの数の合計
  let mut memo = vec![(0usize,1usize);n];
  dfs(&g, &mut memo, 0, 1_000_000_000);
  
  let mut result = vec![0;n];
  dfs2(&g, &memo, &mut result, 0, 1_000_000_000, 0, memo[0].0);

  for v in result {
    println!("{}", v);
  }
}