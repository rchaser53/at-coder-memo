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

// #[derive(Debug, Clone)]
#[derive(Clone)]
enum Next {
  B(usize),
  E(usize)
}

fn main() {
  input!{
    n: usize,
    q: usize,
    vals: [(Usize1, Usize1, Usize1);q]
  }

  let mut last = (0..n as isize).collect::<Vec<isize>>();
  let mut elems = vec![Next::B(0);n];
  for i in 0..n {
    elems[i] = Next::B(i);
  }
  
  for (from, to, i)  in vals {
    match elems[i] {
      // 移動するelemに親がいない
      Next::B(ii) => {
        // -1の場合、elemが存在しないためNext::b(to)
        if last[to] == -1 {
          elems[i] = Next::B(to);
        }
        // それ以外は存在するためNext::e(to)
        else {
          elems[i] = Next::E(last[to] as usize);
        }
        last[to] = last[from];
        last[from] = -1;
      },
      // 移動するelemに親がいる
      Next::E(ii) => {
        // -1の場合、elemが存在しないためNext::b(to)
        if last[to] == -1 {
          elems[i] = Next::B(to);
        }
        // それ以外は存在するためNext::e(to)
        else {
          elems[i] = Next::E(last[to] as usize);
        }
        last[to] = last[from];
        last[from] = ii as isize;
      }
    }
  }
  
  let initial_value = 1_000_000_000;
  let mut result = vec![initial_value;n];
  for v in last {
    if v == -1 { continue }
    let mut i = v as usize;
    if result[i] != initial_value { continue }

    let mut stack = vec![i];
    while let Some(Next::E(ii)) = elems.get(i) {
      i = *ii as usize;
      stack.push(i);
    }
    
    if let Some(Next::B(last)) = elems.get(*stack.last().unwrap()) {
      for ii in stack {
        result[ii] = last + 1;
      }
    } else {
      unreachable!();
    };
  }
  for v in result {
    println!("{}", v);
  }
} 
  