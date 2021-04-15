#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

struct Helper {
  count:usize,
  neighbors:Vec<Vec<usize>>,
  n:usize
}

impl Helper {
  fn dfs(
    &mut self,
    ci: usize,
    set: HashSet<usize>
  ) {
    if set.len() == self.n {
      self.count += 1;
      return
    }
    
    for i in 0..self.neighbors[ci].len() {
      let ni = self.neighbors[ci][i];
      if set.contains(&ni) { continue }
      let mut new_set = set.clone();
      new_set.insert(ni);
      self.dfs(ni, new_set);
    }
  }
}

#[fastout]
fn main() {
  input!{
    n:usize,
    m:usize,
    vals:[(Usize1,Usize1);m]
  }
  
  let mut neighbors = vec![vec![];n];
  for (from, to) in vals {
    neighbors[from].push(to);
    neighbors[to].push(from);
  }
  
  let mut helper = Helper{ n, neighbors, count: 0 };
  let mut set = HashSet::new();
  set.insert(0);
  helper.dfs(0, set);
  
  print!("{}", helper.count);
}