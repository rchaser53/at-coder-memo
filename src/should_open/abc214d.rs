/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

// グループのsizeが必要な場合のunion_find
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

pub fn main(
) {
  input! {
    n:usize,
    mut edges:[(Isize1,Isize1,isize);n-1]
  }

  edges.sort_by(|a,b| a.2.cmp(&b.2));

  let mut memo = vec![-1;n];
  let mut result = 0;
  for (f, t, v) in edges {
    let fv = size(&mut memo, f);
    let tv = size(&mut memo, t);
    result += fv * tv * v;
    connect(&mut memo, f, t);
  }
  println!("{}", result);
}
