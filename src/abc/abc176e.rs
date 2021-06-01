/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::{input};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use superslice::*;
use std::cmp::Ordering;
use num_complex::Complex;
 

pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    m:usize,
    vals:[(Usize1,Usize1);m]
  }

  let mut set = HashSet::new();

  let mut hm = vec![(0,0);h];
  let mut wm = vec![(0,0);w];
  for &(r, c) in &vals {
    hm[r] = (r, hm[r].1 + 1);
    wm[c] = (c, wm[c].1 + 1);

    set.insert((r,c));
  }
  hm.sort_by(|a,b| a.1.cmp(&b.1));
  hm.reverse();
  wm.sort_by(|a,b| a.1.cmp(&b.1));
  wm.reverse();

  let hmax = hm[0].1;
  let wmax = wm[0].1;
  for i in 0..h {
    let (hi, hv) = hm[i];
    if hv != hmax { break }
    for j in 0..w {
      let (wi, wv) = wm[j];
      if wv != wmax { break }
      if !set.contains(&(hi,wi)) {
        println!("{}", hmax+wmax);
        return
      }
    }
  }
  
  println!("{}", hmax+wmax-1);
}