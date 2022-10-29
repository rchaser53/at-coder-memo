/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

// https://atcoder.jp/contests/past202107-open/tasks/past202107_j
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
  input!{
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1);m]
  }
  
  // ここでnodeから出ているedgeの数のcountsを作るのを忘れないこと
  let mut set = HashSet::new();
  let mut g = vec![vec![];n];
  let mut counts = vec![0;n];
  for i in 0..m {
    let (a, b) = edges[i];
    if !set.contains(&(a,b)) {
      g[a].push(b);
      counts[b] += 1;
      set.insert((a,b));
    }
  }

  let result = topological_sort(&g, &mut counts);
  if result.len() == n {
    println!("No");
  } else {
    println!("Yes");
  }
}