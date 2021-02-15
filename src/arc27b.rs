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

fn is_num(a: char) -> bool {
  let u8char = a as u8;
  '0' as u8 <= u8char
    && u8char <= '9' as u8
}

#[fastout]
fn main() {
  input!{
    n:usize,
    mut s1: Chars,
    mut s2: Chars
  }
  
  loop {
    let mut dirty = false;
    for i in 0..n {
      let c1 = s1[i];
      let c2 = s2[i];
      let c1num = is_num(c1);
      let c2num = is_num(c2);
      if c1num && !c2num {
        for ii in 0..n {
          if s1[ii] == c2 {
            s1[ii] = c1;
            dirty = true;
          }
          if s2[ii] == c2 {
            s2[ii] = c1;
            dirty = true;
          }
        }
      } else if !c1num && c2num {
        for ii in 0..n {
          if s1[ii] == c1 {
            s1[ii] = c2;
            dirty = true;
          }
          if s2[ii] == c1 {
            s2[ii] = c2;
            dirty = true;
          }
        }
      } else if c1 != c2 && !c1num && !c2num {
        for ii in 0..n {
          if s1[ii] == c1 {
            s1[ii] = c2;
            dirty = true;
          }
          if s2[ii] == c1 {
            s2[ii] = c2;
            dirty = true;
          }
        }
      }
    }  
    if !dirty { break }
  }
  
  // dbg!(&s1, &s2);
  let mut set = HashSet::new();
  for i in 0..n {
    if !is_num(s1[i]) {
      set.insert(s1[i]);
    }
  }

  let len = set.len();
  if len == 0 {
    println!("1");
    return
  }
  
  if is_num(s1[0]) {
    println!("{}", 10usize.pow(len as u32));
  } else {
    println!("{}", 9 * 10usize.pow((len-1) as u32));
  }
}