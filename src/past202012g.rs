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
  base: Vec<Vec<char>>,
  need: usize,
  result: Option<Vec<(usize, usize)>>,
  y: usize,
  x: usize
}

impl Helper {
  fn culc(
    &mut self,
    mut seen: HashSet<(usize, usize)>,
    mut memo: Vec<(usize, usize)>,
    y: usize,
    x: usize,
  ) {
     
    if self.result.is_some() || seen.contains(&(y, x)) {
      return
    }
    seen.insert((y, x));
    memo.push((y, x));
    
    if memo.len() == self.need {
      self.result = Some(memo);
      return
    }

    if 0 < x && self.base[y][x-1] == '#' {
      self.culc(seen.clone(), memo.clone(), y, x-1);
    }
    
    if x < self.x-1 && self.base[y][x+1] == '#' {
      self.culc(seen.clone(), memo.clone(), y, x+1);
    }
    
    if 0 < y && self.base[y-1][x] == '#' {
      self.culc(seen.clone(), memo.clone(), y-1, x);
    }
    
    if y < self.y-1 && self.base[y+1][x] == '#' {
      self.culc(seen.clone(), memo.clone(), y+1, x);
    }
  }
}

fn main() {
  input!{
    h: usize,
    w: usize,
    base: [Chars;h]
  }
  
  let mut count = 0;
  for i in 0..h {
    for ii in 0..w {
      if base[i][ii] == '#' {
        count += 1;
      }
    }
  }
  
  let mut helper = Helper {
    base,
    need: count,
    result: None,
    y: h,
    x: w
  };
  
  for y in 0..h {
    for x in 0..w {
      if helper.base[y][x] == '#' {
        helper.culc(HashSet::new(), vec![], y, x);
      }
    }
  }
  
  if let Some(result) = helper.result {
    println!("{}", result.len());
    for (y, x) in result {
      println!("{} {}", y+1, x+1);
    }  
  } else {
    println!("0");
  }
}