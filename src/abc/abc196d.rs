#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use superslice::*;

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

struct Helper {
  count:usize,
  num:usize,
  h:usize,
  w:usize,
}

impl Helper {
  fn get_next(&self, r:usize, c:usize) -> (usize, usize) {
    if self.w-1 <= c {
      (r+1, 0)
    } else {
      (r, c+1)
    }
  }
  
  fn dfs(
    &mut self,
    memo: Vec<Vec<bool>>,
    a: usize,
    b: usize,
    r: usize,
    c: usize,
    count: usize
  ) {
    
    if a == 0 && b == 0 {
      if count == self.num {
        self.count += 1;
      }
      return
    }
    if self.h <= r || self.w <= c { return }
    let (new_r, new_c) = self.get_next(r, c);
    if memo[r][c] {
      self.dfs(memo.clone(), a, b, new_r, new_c, count);
    } else {
      let mut new_memo = memo.clone();
      new_memo[r][c] = true;
      if 0 < a {
        if 0 < r && memo[r-1][c] == false {
          let mut nn_memo = new_memo.clone();
          nn_memo[r-1][c] = true;
          self.dfs(nn_memo, a-1, b, new_r, new_c, count+1);
        }
      
        if c < self.w-1 && memo[r][c+1] == false {
          let mut nn_memo = new_memo.clone();
          nn_memo[r][c+1] = true;
          let (new_r, new_c) = self.get_next(r, c+1);
          self.dfs(nn_memo, a-1, b, new_r, new_c, count+1);
        }
        
        if r < self.h-1 && memo[r+1][c] == false {
          let mut nn_memo = new_memo.clone();
          nn_memo[r+1][c] = true;
          self.dfs(nn_memo, a-1, b, new_r, new_c, count+1);
        }
      }
      
      if 0 < b {
        self.dfs(new_memo, a, b-1, new_r, new_c, count+1);
      }
    }
  }
}

fn main() {
  input!{
    h:usize,
    w:usize,
    a:usize,
    b:usize
  }
  
  let mut memo = vec![vec![false;w];h];
  let mut helper = Helper { h, w, num:a+b, count:0 };
  helper.dfs(memo, a, b, 0, 0, 0);
  
  println!("{}", helper.count);
  
}