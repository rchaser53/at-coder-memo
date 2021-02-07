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
use superslice::Ext;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    bx: f64,
    by: f64,
    br: f64,
  }
  let x = (bx * 10000f64).round() as isize;
  let y = (by * 10000f64).round() as isize;
  let r = (br * 10000f64).round() as isize;
  let rr = r * r;
  let from = (x-r+9999).div_euclid(10000);
  let to = (x+r).div_euclid(10000);
  let mut result = 0;
  for i in from..=to {
    let xx = i * 10000 - x;
    let yy = ((rr - xx*xx) as f64).sqrt().ceil() as isize;
    let mut ye = (y+yy).div_euclid(10000);
    let yt = ye * 10000 - y;
    if xx * xx + yt * yt > rr {
      ye -= 1;
    }
    let mut ys = (y-yy+9999).div_euclid(10000);
    let yt = ys * 10000 - y;
    if xx * xx + yt * yt > r * r {
      ys += 1;
    }
    let temp = ye - ys + 1;
    if 0 < temp {
      result += temp;
    }
  }
  
  println!("{}", result);
}