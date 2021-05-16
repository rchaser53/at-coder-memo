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

fn convert(c: char) -> usize {
  if 'A' <= c && c <= 'Z' {
    10 + c as usize - 'A' as usize
  } else {
    c as usize - '0' as usize
  }
}

#[fastout]
fn main() {
  input!{
    n: usize,
    s1: Chars,
    s2: Chars
  }
  
  let mut unknown = HashSet::new();
  let mut components = UnionFind::new(36);
  for i in 0..n {
    let v1 = convert(s1[i]);
    let v2 = convert(s2[i]);
    components.union(v1, v2);
    if 10 <= v1 {
      unknown.insert(s1[i]);
    }
    if 10 <= v2 {
      unknown.insert(s2[i]);
    }
  }
  
  let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  let digit = "0123456789";
  for d in digit.chars() {
    for c in alphabet.chars() {
      let dv = convert(d);
      let cv = convert(c);
      if components.find(dv) == components.find(cv) {
        unknown.remove(&c);
      }
    }
  }
  
  let mut count = 0;
  for c in alphabet.chars() {
    let v = convert(c);
    if unknown.contains(&c) && components.find(v) == v {
      count += 1;
    }
  }
  
  if unknown.contains(&s1[0]) {
    println!("{}", 9 * 10usize.pow((count-1) as u32));
  } else {
    println!("{}", 10usize.pow(count as u32));
  }
}