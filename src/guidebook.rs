#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    base: [(String, usize);n]
  }
  
  let mut vals = vec![];
  for i in 0..n {
    let (v, c) = base[i].clone(); 
    vals.push((v, c, i+1));
  }
  
  vals.sort_by(|a,b| {
    let v = a.0.cmp(&b.0);
    if v == Ordering::Equal {
      let vv = a.1.cmp(&b.1);
      if vv == Ordering::Greater {
        Ordering::Less
      } else {
        Ordering::Greater
      }
    } else {
      v
    }
  });
  
  for (_, _, i) in vals {
    println!("{}", i);
  }
}