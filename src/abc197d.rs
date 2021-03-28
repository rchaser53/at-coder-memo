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

// 正Nは偶数なので、p0とpN/2の中点を取ることで正N角形の中心が得られる
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
  // (cosθ,sinθ)が単位ベクトルになるので以下でも良さそう
  // let base = Complex::new(theta.cos(), theta.sin());
  
  // 複素数のベクトルはかけると長さが積になり、角度が和になる
  let c1 = mid + base * (c0 - mid);
  
  println!("{} {}", c1.re, c1.im);
}

fn answer2() {
  input!{
    n:f64,
    x0:f64,
    y0:f64,
    x2:f64,
    y2:f64
  }
  
  let xo = (x0+x2) / 2f64;
  let yo = (y0+y2) / 2f64;
  let ox0 = x0 - xo;
  let oy0 = y0 - yo;
  let theta = 2f64 * std::f64::consts::PI / n;

  // θ、ベクトルの向きを変える(座標を移動させる)やつ
  // x' = x*cosθ - y*sinθ
  // y' = x*sinθ + y*cosθ
  let ox1 = ox0 * theta.cos() - oy0 * theta.sin();
  let oy1 = ox0 * theta.sin() + oy0 * theta.cos();
  println!("{} {}", ox1 + xo, oy1 + yo);
}
