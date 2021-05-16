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

fn main() {
  input!{
    n: usize,
    q: usize
  }
  
  let mut rows = (0..n).collect::<Vec<usize>>();
  let mut columns = (0..n).collect::<Vec<usize>>();
  
  let mut flag = false;
  for _ in 0..q {
    input! {
      t: usize
    }
    match t {
      1 => {
        input! {
          a: Usize1,
          b: Usize1
        }
        rows.swap(a, b);
      },
      2 => {
        input! {
          a: Usize1,
          b: Usize1
        }
        columns.swap(a,b);
      },
      3 => {
        flag = !flag;
        std::mem::swap(&mut rows, &mut columns);
      },
      _ => {
        input! {
          a: Usize1,
          b: Usize1
        }

        if flag {
          println!("{}", n * columns[b] + rows[a]);
        } else {
          println!("{}", n * rows[a] + columns[b]);
        }
      }
    }
  }
} 
  