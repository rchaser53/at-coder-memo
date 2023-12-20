/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

struct Helper {
  children: Vec<usize>,
  g: Vec<Vec<usize>>,
  result: usize
}

impl Helper {
  fn dfs(&mut self, ci:usize, li:usize) -> usize {
    let mut result = 1;

    if ci == 0 {
      let mut temp = 0;
      let mut tot = 0;
      for i in 0..self.g[ci].len() {
        let ni = self.g[ci][i];
        let v = self.dfs(ni, ci);
        temp = temp.max(v);
        tot += v;
      }

      self.result = tot - temp + 1;
    } else {
      for i in 0..self.g[ci].len() {
        let ni = self.g[ci][i];
        if ni != li {
          result += self.dfs(ni, ci);
        }
      }
      self.children[ci] = result;
    }
    result
  }
}

fn main() {
  input! {
    n:usize,
    uv:[(Usize1,Usize1);n-1]
  }

  let mut g = vec![vec![];n];
  for (u,v) in uv {
    g[u].push(v);
    g[v].push(u);
  }

  let mut helper = Helper { children: vec![0;n], g, result: 1 };
  helper.dfs(0, 1_000_000);
  println!("{}", helper.result);
}