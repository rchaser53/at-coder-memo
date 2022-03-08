/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

struct Helper {
  result: Vec<(usize, usize)>,
  g: Vec<Vec<usize>>,
  now:usize
}

impl Helper {
  fn dfs(&mut self, ci:usize, last:usize) {
    self.result[ci].0 = self.now;
    let len = self.g[ci].len();
    for i in 0..len {
      let ni = self.g[ci][i];
      if ni == last { continue }
      self.dfs(ni, ci);
    }

    if len == 1 && ci != 0 {
      self.now += 1;
    }
    self.result[ci].1 = self.now - 1;
  }
}

fn main() {
  input! {
    n:usize,
    edges:[(Usize1,Usize1);n-1]
  }

  let mut g = vec![vec![];n];
  for (a,b) in edges {
    g[a].push(b);
    g[b].push(a);
  }

  let mut helper = Helper { g, result:vec![(0,0);n], now:1 };
  helper.dfs(0, 1_000_000_000);
  for (a, b) in helper.result {
    println!("{} {}", a, b);
  }
}