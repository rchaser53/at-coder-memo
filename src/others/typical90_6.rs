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
use superslice::Ext;

const MOD:usize = 998244353;
const MAX:usize = 200_000;

fn main() {
  input!{
    n:usize,
    k:usize,
    s:Chars
  }
  let mut memo = vec![vec![];26];
  for i in 0..n {
    let c = (s[i] as u8 - 'a' as u8) as usize;
    memo[c].push(i);
  }
  let mut result = String::from("");
  let mut ci = 0;
  for i in 0..k {
    let left = k-i;
    let max = n-left;
    for j in 0..26 {
      let mli = memo[j].len();
      
      let ti = match memo[j].binary_search(&ci) {
        Ok(ti) => ti,
        Err(ti) => ti
      };
      if ti == mli || max < memo[j][ti] { continue }
      let ri = memo[j][ti];
      ci = ri+1;
      result.push_str(&s[ri].to_string());
      break
    }
  }
  println!("{}", result);
}