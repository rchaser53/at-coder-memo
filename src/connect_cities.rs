#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashMap, HashSet};

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


fn main() {
  input! {
    n: usize,
    m: usize,
    vals: [(Isize1, Isize1);m]
  }
  
  let mut parents: Vec<isize> = vec![-1;n];
  
  for (a, b) in vals.iter() {
    connect(&mut parents, *a, *b);
  }
  for (a, b) in vals {
    connect(&mut parents, a, b);
  }
  
  let mut map: HashMap<isize, HashSet<usize>> = HashMap::new();
  for (i, v) in parents.into_iter().enumerate() {
    if v < 0 {
      let entry = map.entry(i as isize).or_insert(HashSet::new());
      entry.insert(i);
    } else {
      let entry = map.entry(v as isize).or_insert(HashSet::new());
      entry.insert(i);
    }
  }
  println!("{}", map.keys().len() - 1);
}