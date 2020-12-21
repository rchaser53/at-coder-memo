#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

// use Euclidean Algorithm
// ax+by=g => bxx+ryy=g => bxx+(a-a/b*b)yy=g => ayy+b(xx-a/b*yy)=g
fn ext_gcd(
  a: isize,
  b: isize
)
// (g, x, y)
-> (isize, isize, isize) {
  if b == 0 {
    (a, 1, 0)
  } else {
    let (g, x, y) = ext_gcd(b, a % b);
    (g, y, x-a/b*y)
  }
}

fn solve() {
  input!{
    mut n: isize,
    mut s: isize,
    mut k: isize
  }
  let (g, x, y) = ext_gcd(n, k);
  if s % g != 0 {
    println!("-1");
    return
  }
  n /= g;
  s /= g;
  //  k /= g;
  // (n-s)/k = (n-s)*y%n
  println!("{}", ((n-s) * y % n + n) % n);
}

fn main() {
  input!{
    t: usize,
  }
  
  for i in 0..t {
    solve();
  }
}