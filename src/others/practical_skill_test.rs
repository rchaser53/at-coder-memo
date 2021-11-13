#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    h: usize,
    w: usize,
    d: isize,
    rows: [[Usize1;w];h],
    q: usize,
    vals: [(Usize1, Usize1);q]
  }
  
  let ud = d as usize;
  let mut map = vec![(0,0);h*w];
  for i in 0..h {
    for ii in 0..w {
      map[rows[i][ii]] = (i as isize + 1, ii as isize + 1);
    }
  }
  
  let limit = h * w / ud + 1;
  let mut memo = vec![vec![0;limit];ud];
  for i in 0..ud {
    for ii in 1..limit {
      let ni = i+ii*ud;
      if h*w <= ni { break }
      let (x1, y1) = map[i+(ii-1)*ud];
      let (x2, y2) = map[i+ii*ud];
      memo[i][ii] = memo[i][ii-1] + (x1 - x2).abs() + (y1 - y2).abs();
    }
  }
  
  for i in 0..q {
    let (from, to) = vals[i];
    if from == to {
      println!("0");
    } else {
      let ii = from % ud;
      let from_i = from / ud;
      let to_i = to / ud;
      println!("{}", memo[ii][to_i] - memo[ii][from_i]);
    }
  }
}
