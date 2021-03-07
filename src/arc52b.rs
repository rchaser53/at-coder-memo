#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use num::Num;
use std::f64::consts::PI;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n:usize,
    q:usize,
    vals:[(f64,f64,f64);n],
    queries:[(f64,f64);q]
  }
  
  for (from, to) in queries {
    let mut result = 0f64;
    for i in 0..n {
      let (bottom, r, height) = vals[i];
      let top = bottom + height;
      if top <= from || to <= bottom { continue }      
      if bottom < from {
        if top <= to {
          let real_height = top - from;
          let rr = real_height / height * r;
          result += PI * rr * rr * real_height / 3f64;
        }
        else {
          let base_height = top - from;
          let rr = base_height / height * r;
          let v = PI * rr * rr * base_height / 3f64;
          let minus_height = top - to;
          let rr2 = minus_height / height * r;
          let mv = PI * rr2 * rr2 * minus_height / 3f64;
          result += v - mv;
        }
      } else {
        if top <= to {
          result += PI * r * r * height / 3f64;
        }
        else {
          let minus_height = top - to;
          let rr = minus_height / height * r;
          let v = PI * r * r * height / 3f64;
          let mv = PI * rr * rr * minus_height / 3f64;
          result += v - mv;
        }
      }
    }
    println!("{}", result);
  }
}