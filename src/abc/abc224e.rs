/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    h:usize,
    w:usize,
    n:usize,
    points:[(Usize1,Usize1,usize);n]
  }

  let mut btree_map = BTreeMap::new();
  for i in 0..n {
    btree_map.entry(points[i].2).or_insert(vec![]).push(i);
  }
  let mut rmax = vec![0;h];
  let mut cmax = vec![0;w];
  let mut result = vec![0;n];
  for arr in btree_map.values().rev() {
    for &i in arr {
      result[i] = std::cmp::max(rmax[points[i].0], cmax[points[i].1]);
    }
    for &i in arr {
      let r = points[i].0;
      let c = points[i].1;
      rmax[r] = std::cmp::max(rmax[r], result[i]+1);
      cmax[c] = std::cmp::max(cmax[c], result[i]+1);
    }
  }

  for v in result {
    println!("{}", v);
  }
}