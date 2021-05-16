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

fn helper<T: Iterator<Item=char>>(
  def: &mut T,
  mut result: Vec<char>,
) -> Vec<char>  {
  if let Some(c) = def.next() {
    if c == ')' {
      let mut cloned = result.clone();
      cloned.reverse();
      result.append(&mut cloned);
      result
    } else if c == '(' {
      let mut new = helper(def, vec![]);
      result.append(&mut new);
      helper(def, result)
    } else {
      result.push(c);
      helper(def, result)
    }
  } else {
    result
  }
}

fn main() {
  input!{
    s: Chars
  }
  
  let result = helper(&mut s.into_iter(), vec![]);
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<String>());
}
