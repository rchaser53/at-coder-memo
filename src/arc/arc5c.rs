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
use std::cmp::*;

const MOD:usize = 1_000_000_007;

struct Helper {
  maze: Vec<Vec<char>>,
  seen: HashMap<(usize, usize),usize>,
  succeed: bool,
  h: usize,
  w: usize
}

impl Helper {
  fn dfs(
    &mut self,
    r: usize,
    c: usize,
    mut life: usize
  ) {
    if self.succeed { return }
    if self.maze[r][c] == '#' {
      if life == 0 { return }
      life -= 1;
    }
    
    if let Some(v) = self.seen.get(&(r, c)) {
      if life <= *v { return }
    }
    self.seen.insert((r,c), life);
    
    if self.maze[r][c] == 'g' {
      self.succeed = true;
      return
    }
  
    if 0 < r {
      self.dfs(r-1, c, life);
    }
    
    if r < self.h-1 {
      self.dfs(r+1, c, life);
    }
    
    if 0 < c {
      self.dfs(r, c-1, life);
    }
    
    if c < self.w-1 {
      self.dfs(r, c+1, life);
    }
  }
}



fn main() {
  input!{
    h: usize,
    w: usize,
    maze: [Chars;h]
  }
  
  let mut helper = Helper{
    maze,
    seen: HashMap::new(),
    h,
    w,
    succeed: false
  };
  
  for i in 0..h {
    for ii in 0..w {
      if helper.maze[i][ii] == 's' {
        helper.dfs(i, ii, 2);
        if helper.succeed {
          println!("YES");
        } else {
          println!("NO");
        }
        return
      }
    }
  }
}