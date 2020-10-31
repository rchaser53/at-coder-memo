#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

fn size(parents: &mut Vec<isize>, i: isize) -> isize {
  let ii = find(parents, i) as usize;
  -1 * parents[ii]
}

fn find(parents: &mut Vec<isize>, i: isize) -> isize {
  let ii = i as usize;
  if parents[ii] < 0 {
    i
  } else {
    parents[ii] = find(parents, parents[ii]);
    parents[ii]
  }
}

fn connect(
  parents: &mut Vec<isize>,
  a: isize,
  b: isize
) -> bool {
  let mut pa = find(parents, a);
  let mut pb = find(parents, b);
  
  if pa == pb { return false }
  
  if size(parents, pa) < size(parents, pb) {
    let temp = pa;
    pa = pb;
    pb = temp;
  }
  
  let paa = pa as usize;
  let pbb = pb as usize;
  parents[paa] += parents[pbb];
  parents[pbb] = pa;
  
  true
}

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    mut vals: [i128;n],
    mut converts: [i128;n],
    nodes: [(Isize1, Isize1);m]
  }
  
  let mut parents: Vec<isize> = vec![-1;n];
  for (a, b) in nodes.iter() {
    connect(&mut parents, *a, *b);
  }
  for (a, b) in nodes {
    connect(&mut parents, a, b);
  }
  
  let mut original = vec![0;n];
  let mut converted = vec![0;n];
  for i in 0..n {
    let v = parents[i];
    if v < 0 {
      original[i] += vals[i];
      converted[i] += converts[i];
    } else {
      original[v as usize] += vals[i];
      converted[v as usize] += converts[i];
    }
  }
    
  for i in 0..n {
    if original[i] != converted[i] {
      println!("No");
      return
    }
  }

  println!("Yes");
}