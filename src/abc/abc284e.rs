/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

const LIMIT:usize = 1_000_000;
struct Helper {
  done: bool,
  g: Vec<Vec<usize>>,
}

impl Helper {
  fn dfs(&mut self, memo: &mut Vec<bool>, ci:usize) -> usize {
    if self.done {
      return LIMIT
    }

    let mut result = 1;
    for i in 0..self.g[ci].len() {
      let ni = self.g[ci][i];
      if !memo[ni] {
        memo[ni] = true;
        result += self.dfs(memo, ni);
        memo[ni] = false;
      }
    }

    if result >= LIMIT {
      self.done = true;
    }

    result
  }
}

fn main() {
  input! {
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1);m]
  }

  let mut g = vec![vec![];n];
  for (a,b) in edges {
    g[a].push(b);
    g[b].push(a);
  }
  let mut helper = Helper { done: false, g };
  let mut memo = vec![false;n];
  memo[0] = true;
  let result = helper.dfs(&mut memo, 0);
  if helper.done {
    println!("{}", LIMIT);
  } else {
    println!("{}", result);
  }
}