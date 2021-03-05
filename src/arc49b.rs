#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use superslice::*;
use itertools::Itertools;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use num::Num;

const MOD:usize = 1_000_000_007;

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
  T: std::marker::Copy
    + std::ops::Add<T, Output = T>
    + std::ops::Sub<T, Output = T>
    + std::ops::Div<T, Output = T>
    + std::cmp::PartialOrd<T>
    + std::convert::TryFrom<i32>,
  F: Fn(T) -> std::cmp::Ordering,
{
  let two = T::try_from(2).ok().unwrap();
  while end - begin >= epsilon {
    let mid = begin + (end - begin) / two;

    match f(mid) {
      Ordering::Less => {
        begin = mid + epsilon;
      }
      _ => {
        end = mid;
      }
    }
  }

  begin
}

fn main() {
  input!{
    n: usize,
    vals: [(f64, f64, f64);n]
  }
  
  let result = lower_bound(0.0, 1e9, 0.0001, |v| {
    let left = vals.iter().copied().map(|(x, _, c)| x-v/c)
                   .max_by_key(|&f| ordered_float::OrderedFloat(f))
                   .unwrap();
    let right = vals.iter().copied().map(|(x, _, c)| x+v/c)
                   .min_by_key(|&f| ordered_float::OrderedFloat(f))
                   .unwrap();
    let bottom = vals.iter().copied().map(|(_, y, c)| y-v/c)
                   .max_by_key(|&f| ordered_float::OrderedFloat(f))
                   .unwrap();
    let top = vals.iter().copied().map(|(_, y, c)| y+v/c)
                   .min_by_key(|&f| ordered_float::OrderedFloat(f))
                   .unwrap();
    if left <= right && bottom <= top {
      Ordering::Greater
    } else {
      Ordering::Less
    }
  });
  
  println!("{}", result);
}