#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    s: String,
  }
  
  let mut map = HashMap::new();
  let mut v = String::from("b");
  map.insert(v.clone(), 0);
  
  for i in 1..=100 {
    match i % 3 {
      1 => {
        v = format!("a{}c", v);
      },
      2 => {
        v = format!("c{}a", v);
      },
      _ => {
        v = format!("b{}b", v);
      }
    }
    map.insert(v.clone(), i);
  }
  
  if let Some(v) = map.get(&s) {
    println!("{}", v);
  } else {
    println!("-1");
  }
}
