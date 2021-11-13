#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    h: usize,
    w: usize,
    n: usize,
    m: usize,
    lights: [(Usize1, Usize1);n],
    blocks: [(Usize1, Usize1);m],
  }
  
  let mut maze = vec![vec![0;w];h];
  let mut rows = vec![vec![];h];
  let mut columns = vec![vec![];w];
  for (r, c) in lights {
    rows[r].push((1, c));
    columns[c].push((1, r));
  }
  for (r, c) in blocks {
    rows[r].push((2, c));
    columns[c].push((2, r));
  }
  
  for i in 0..h {
    rows[i].sort_by(|a,b| a.1.cmp(&b.1));
  }
  for i in 0..w {
    columns[i].sort_by(|a,b| a.1.cmp(&b.1));
  }
  
  for r in 0..h {
    let row = &rows[r];
    let mut index = 0;
    let mut last = 0;
    for i in 0..row.len() {
      let (v, until) = row[i];
      for ii in index..=until {
        if v == 1 || last == 1 {
          maze[r][ii] = 1;        
        }
      }
      if v == 2 {
        maze[r][until] = 2;
      }
      index = until+1;
      last = v;
    }
    
    for i in index..w {
      if last == 1 {
        maze[r][i] = 1;
      }
    }
  }
  
  for c in 0..w {
    let column = &columns[c];
    let mut index = 0;
    let mut last = 0;
    for i in 0..column.len() {
      let (v, until) = column[i];
      for ii in index..=until {
        if v == 1 || last == 1 {
          maze[ii][c] = 1;
        }
      }
      if v == 2 {
        maze[until][c] = 2;
      }
      index = until+1;
      last = v;
    }
    
    for i in index..h {
      if last == 1 {
        maze[i][c] = 1;
      }
    }
  }
  
  let mut count = 0;
  for i in 0..h {
    for ii in 0..w {
      if maze[i][ii] == 1 {
        count += 1;
      }
    }
  }
  println!("{}", count);
}