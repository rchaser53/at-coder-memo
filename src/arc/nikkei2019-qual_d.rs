/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

// トポロジカルソート(topological sort) DFS
// ループの検出ができない
fn topological_sort(
  g: &Vec<Vec<usize>>,
  seen: &mut Vec<bool>,
  result: &mut Vec<usize>,
  ci: usize,
) {
  seen[ci] = true;
  for &ni in &g[ci] {
    if !seen[ni] {
      topological_sort(g, seen, result, ni);
    }
  }
  result.push(ci);
}

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1);n-1+m]
  }

  let mut g = vec![vec![];n];
  let mut rg = vec![vec![];n];
  for (f, t) in edges {
    g[f].push(t);
    rg[t].push(f);
  }

  let mut seen = vec![false;n];
  let mut topo = vec![];
  for i in 0..n {
    if !seen[i] {
      topological_sort(&g, &mut seen, &mut topo, i);
    }
  }
  topo.reverse();
  let mut for_sort = vec![0;n];

  for i in 0..n {
    for_sort[topo[i]] = i;
  }
  for i in 0..n {
    rg[i].sort_by(|&a,&b| for_sort[a].cmp(&for_sort[b]));
  }
  let mut result = vec![0;n];

  for i in 0..n {
    if !rg[i].is_empty() {
      result[i] = rg[i][rg[i].len()-1] + 1;
    }
  }

  for v in result {
    println!("{}", v);
  }
}
