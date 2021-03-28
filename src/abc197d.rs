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
use superslice::*;
use std::cmp::Ordering;
use num_complex::Complex;

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

fn main() {
  input!{
    n:f64,
    p0:(f64, f64),
    pn2:(f64, f64)
  }
  let c0 = Complex::new(p0.0, p0.1);
  let c2 = Complex::new(pn2.0, pn2.1);
  
  // 中点
  let mid = (c0 + c2) / Complex::new(2f64, 0f64);
  
  // 正n角形の内角
  let theta = 2f64 * std::f64::consts::PI / n;
  
  // θを加えるためだけの単位ベクトル的なものを作る
  let base = Complex::from_polar(&1f64, &theta);
  
  // 複素数のベクトルはかけると長さが積になり、角度が和になる
  let c1 = mid + base * (c0 - mid);
  
  println!("{} {}", c1.re, c1.im);
}