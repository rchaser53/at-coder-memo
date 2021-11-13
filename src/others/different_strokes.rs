#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [(isize, isize);n]
  }
  
  let mut stack = vec![(0,0);n];
  for i in 0..n {
    let (l, r) = vals[i];
    stack[i] = (i, l+r);
  }
  
  let mut a = 0;
  let mut b = 0;
  stack.sort_by(|a,b| a.1.cmp(&b.1));
  stack.reverse();
  for i in 0..n {
    if i % 2 == 0 {
      a += vals[stack[i].0].0;
    } else {
      b += vals[stack[i].0].1;
    }
  }
  
  println!("{}", a - b);
}

fn culc(
  n: usize,
  a: &Vec<(usize, isize)>,
  b: &Vec<(usize, isize)>,
) -> isize {
  let mut set = HashSet::new();
  let mut av = vec![];
  let mut bv = vec![];
  let mut ai = 0;
  let mut bi = 0;
  let mut flag = true;
  while ai < n || bi < n {
    if flag {
      flag = !flag;
      while ai < n && set.contains(&a[ai].0) {
        ai += 1;
      }
      if n <= ai { continue }
      av.push(a[ai].1);
      set.insert(a[ai].0);
    } else {
      flag = !flag;
      while bi < n && set.contains(&b[bi].0) {
        bi += 1;
      }
      if n <= bi { continue }
      bv.push(b[bi].1);
      set.insert(b[bi].0);
    }
  }
  av.into_iter().sum::<isize>() - bv.into_iter().sum::<isize>()
}

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [(isize, isize);n]
  }
  
  let mut base = vec![(0,0);n];
  let mut a = base.clone();
  let mut b = a.clone();
  for i in 0..n {
    base[i] = (i, vals[i].0 + vals[i].1);
  }
  base.sort_by(|a,b| a.1.cmp(&b.1));
  base.reverse();
  for i in 0..n {
    a[i] = (base[i].0, vals[base[i].0].0);
    b[i] = (base[i].0, vals[base[i].0].1);
  }
  
  println!("{}", culc(n, &a, &b));
}