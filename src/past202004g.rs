#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    q: usize,
  }
  
  let mut que = VecDeque::new();
  for _ in 0..q {
    input!{
      t: usize,
    }
    
    if t == 1 {
      input!{
        c: char,
        v: usize
      }
      que.push_back((c,v));
    } else {
      input!{
        need: usize
      }
      
      let mut memo = vec![0;26];
      let mut i = 0;
      while let Some(elem) = que.get_mut(0) {
        let ti = (elem.0 as u8 - 'a' as u8) as usize;
        i += elem.1;
        if need < i {
          let left = i - need;
          memo[ti] += (elem.1 - left);
          elem.1 = left;
          break
        } else if need == i {
          memo[ti] += elem.1;
          que.pop_front().unwrap();
          break
        } else {
          memo[ti] += elem.1;
          que.pop_front().unwrap();
        }
      }
      
      let mut result = 0;
      for i in 0..memo.len() {
        result += memo[i] * memo[i];
      }
      println!("{}", result);
    }
  }  
}
