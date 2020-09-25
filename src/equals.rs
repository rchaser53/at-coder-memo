#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashSet, HashMap};

fn size(
  parents: &mut Vec<isize>,
  i: isize
) -> isize {
  let ii = find(parents, i) as usize;
  -1 * parents[ii]
}

fn find(
  parents: &mut Vec<isize>,
  i: isize
) -> isize {
  let ui = i as usize;
  if -1 < parents[ui] {
    parents[ui] = find(parents, parents[ui]);
    parents[ui]
  } else {
    i
  }
}

fn connect(
  parents: &mut Vec<isize>,
  a: isize,
  b: isize,
) -> bool {
  let pa = find(parents, a);
  let pb = find(parents, b);
  
  if pa == pb { return false }
  
  let av = size(parents, pa);
  let bv = size(parents, pb);
  
  let upa = pa as usize;
  let upb = pb as usize;
  if av < bv {
    parents[upb] += parents[upa];
    parents[upa] = pb;
  } else {
    parents[upa] += parents[upb];
    parents[upb] = pa;
  }
  true
}

fn main() {
  input! {
    n: usize,
    m: usize,
    vals: [Usize1;n],
    queries: [(Isize1, Isize1);m]
  }
  
  let mut parents: Vec<isize> = vec![-1;n];
  for (a, b) in queries.iter() {
    connect(&mut parents, *a, *b);
  }
  for (a, b) in queries {
    connect(&mut parents, a, b);
  }
  
  let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
  
  for i in 0..n {
    let v = parents[i];
    if v < 0 {
      let entry = map.entry(i).or_insert(HashSet::new());
      entry.insert(i);
    } else {
      let uv = v as usize;
      let entry = map.entry(uv).or_insert(HashSet::new());
      entry.insert(i);
    }
  }
  
  let mut result = 0;
  for (_, v) in map {
    for vv in v.iter() {
      let vvv = vals[*vv];
      if v.contains(&vvv) {
        result += 1;
      }
    }
  }
  
  println!("{}", result);
}