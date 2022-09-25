/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

fn dfs(g:&Vec<Vec<usize>>, arr:&mut Vec<usize>, gi:usize, ci:usize, li:usize) {
  if gi == ci {
    println!("{}", arr.iter().map(|v| (v+1).to_string()).collect::<Vec<String>>().join(" "));
    return
  }

  for &ni in &g[ci] {
    if ni != li {
      arr.push(ni);
      dfs(g, arr, gi, ni, ci);
      arr.pop();
    }
  }
}

fn main() { 
  input! { 
    n:usize,
    x:Usize1,
    y:Usize1,
    edges:[(Usize1,Usize1);n-1]
  }
  let mut g = vec![vec![];n];
  for (a,b) in edges {
    g[a].push(b);
    g[b].push(a);
  }

  dfs(&g, &mut vec![x], y, x, 1_000_000_000);
}