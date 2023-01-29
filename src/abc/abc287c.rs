/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

struct Helper {
  result: bool,
  g: Vec<Vec<usize>>,
}

impl Helper {
  fn dfs(&mut self, ci:usize, li:usize, count:usize) {
    if self.g[ci].len() == 1 && self.g[ci][0] == li {
      if count == self.g.len() {
        self.result = true;
      }
      return
    }

    for i in 0..self.g[ci].len() {
      let ni = self.g[ci][i];
      if ni == li { continue }
      self.dfs(ni, ci, count+1);
    }
  }
}

fn main() {
  input!{
    n:usize,
    m:usize,
    ab:[(Usize1,Usize1);m]
  }

  if n-1 != m {
    println!("No");
    return
  }

  let mut g = vec![vec![];n];
  for (a,b) in ab {
    g[a].push(b);
    g[b].push(a);
  }

  let mut helper = Helper {
    g, result: false
  };
  for i in 0..n {
    if helper.g[i].len() == 1 {
      helper.dfs(i, 1_000_000_000, 1);
      break
    }
  }

  if helper.result {
    println!("Yes");
  } else {
    println!("No");
  }
}