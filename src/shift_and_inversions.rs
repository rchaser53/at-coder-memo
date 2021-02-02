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
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  let mut count = 0;
  let mut memo = vec![0;n+1];
  for i in 0..n {
    add(&mut memo, vals[i] + 1, 1);
    count += i + 1 - sum(&memo, vals[i] + 1);
  }
  
  println!("{}", count);
  for i in 0..n-1 {
    let v = vals[i];
    count += n - 2 * v - 1;
    println!("{}", count);
  }
}

fn add(bit:&mut Vec<usize>, i:usize, x:usize) {
  let mut j = i;
  while j < bit.len() {
    bit[j] += x;
    j += lsb(j);
  }
}

fn sum(bit: &Vec<usize>, i:usize) -> usize {
  if i == 0 {
    0
  } else {
    bit[i] + sum(bit, i - lsb(i))
  }
}

fn lsb(i: usize) -> usize {
  ((i as isize) & -(i as isize)) as usize
}