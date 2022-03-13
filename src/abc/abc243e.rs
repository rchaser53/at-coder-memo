/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input!{
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1,usize);m]
  }

  let mut g = vec![vec![10usize.pow(15);n];n];
  for &(a,b,c) in &edges {
    g[a][b] = c;
    g[b][a] = c;
  }

  for k in 0..n {
    for i in 0..n {
      for j in 0..n {
        g[i][j] = std::cmp::min(g[i][j], g[i][k] + g[k][j]);
      }
    }
  }

  let mut result = 0;
  for (a,b,c) in edges {
    for i in 0..n {
      if g[a][i] + g[i][b] <= c {
        result += 1;
        break
      }
    }
  }

  println!("{}", result);
}