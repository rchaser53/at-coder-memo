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

const MOD:usize = 1_000_000_007;

// 垂線引くやつ
fn culc1(
  a: &(f64, f64),
  b: &(f64, f64),
  c: &(f64, f64),
  r: f64
) -> bool {
  let ab = (b.0 - a.0, b.1 - a.1);
  let ac = (c.0 - a.0, c.1 - a.1);
  let k = (ab.0 * ac.0 + ab.1 * ac.1) / (ab.0.powi(2) + ab.1.powi(2));
  let len = (k * ab.0 - ac.0).powi(2) + (k * ab.1 - ac.1).powi(2);
  r <= len
}

fn culc2(
  a: &(f64, f64),
  b: &(f64, f64),
  r: f64
) -> bool {
  let len = (b.0 - a.0).powi(2) + (b.1 - a.1).powi(2);
  len <= r
}

#[fastout]
fn main() {
  input!{
    x1:f64, y1:f64, r:f64,
    x2:f64, y2:f64, x3:f64, y3:f64
  }
  
  let a = (x2, y2);
  let b = (x2, y3);
  let c = (x3, y3);
  let d = (x3, y2);
  let o = (x1, y1);
  let rr = r*r;
  let inside_box =
    culc1(&a, &b, &o, rr) &&
    culc1(&b, &c, &o, rr) &&
    culc1(&c, &d, &o, rr) &&
    culc1(&d, &a, &o, rr) &&
    x2 < x1 && x1 < x3 &&
    y2 < y1 && y1 < y3;
  
  let inside_circle =
    culc2(&a, &o, rr) &&
    culc2(&b, &o, rr) &&
    culc2(&c, &o, rr) &&
    culc2(&d, &o, rr);
  
  if inside_box {
    println!("NO");
    println!("YES");
  } else if inside_circle {
    println!("YES");
    println!("NO");
  } else {
    println!("YES");
    println!("YES");
  }
}