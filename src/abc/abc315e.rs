/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn dfs(g: &Vec<Vec<usize>>, result: &mut Vec<usize>, seen: &mut Vec<bool>, ci:usize) {
  for i in 0..g[ci].len() {
    let ni = g[ci][i];
    if seen[ni] { continue }
    dfs(g, result, seen, ni);
  }
  seen[ci] = true;
  result.push(ci + 1);
}

fn main() {
  input! {
    n:usize,
  }

  let mut g = vec![vec![];n];
  for i in 0..n {
    input! {
      c:usize,
      p:[Usize1;c]
    }
    g[i] = p;
  }

  let mut result = vec![];
  dfs(&g, &mut result, &mut vec![false;n], 0);
  result.pop();

  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));  
}