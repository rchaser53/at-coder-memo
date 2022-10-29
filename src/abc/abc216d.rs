use proconio::input;
use proconio::marker::*;
use std::collections::*;

// https://atcoder.jp/contests/abc216/tasks/abc216_d
// トポロジカルソート( topological sort )
// ループの検出ができる(result.len() == nで判断)
fn topological_sort(
  g: &Vec<Vec<usize>>,
  counts: &mut Vec<usize>,
) -> Vec<usize> {
  let mut result = vec![];
  let mut zeros = vec![];

  for i in 0..g.len() {
    if counts[i] == 0 {
      zeros.push(i);
    }
  }

  while let Some(ci) = zeros.pop() {
    result.push(ci);
    for &ni in &g[ci] {
      counts[ni] -= 1;
      if counts[ni] == 0 {
        zeros.push(ni);
      }
    }
  }
  result
}
fn main() {
  input! {
    n:usize,
    m:usize,
  }

  let mut g = vec![vec![];n];
  let mut counts = vec![0;g.len()];
  for _ in 0..m {
    input! {
      k:usize,
      vals:[Usize1;k]
    }
    
    for j in 0..k-1 {
      let ci = vals[j];
      let ni = vals[j+1];
      g[ci].push(ni);
      counts[ni] += 1;
    }
  }

  let result = topological_sort(&g, &mut counts);
  if result.len() == n {
    println!("Yes");
  } else {
    println!("No");
  }
}