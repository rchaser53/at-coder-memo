use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

// グループのsizeが必要な場合の union_find
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
  input!{
    n:usize,
    vals:[(isize,isize);n]
  }

  let mut x = vec![];
  let mut y = vec![];
  for i in 0..n {
    let (xv, yv) = vals[i];
    x.push((xv, i));
    y.push((yv, i));
  }
  x.sort();
  y.sort();
  let mut edges = vec![];
  for i in 1..n {
    let a = x[i-1].1;
    let b = x[i].1;
    let c = x[i].0 - x[i-1].0;
    edges.push((a,b,c));
  }
  for i in 1..n {
    let a = y[i-1].1;
    let b = y[i].1;
    let c = y[i].0 - y[i-1].0;
    edges.push((a,b,c));
  }
  edges.sort_by(|a,b| a.2.cmp(&b.2));
  let mut memo = vec![-1;n];
  let mut result = 0;
  for (a, b, c) in edges {
    let ai = a as isize;
    let bi = b as isize;
    if find(&mut memo, ai) == find(&mut memo, bi) {
      continue
    }
    connect(&mut memo, ai, bi);
    result += c;
  }
  println!("{}", result);
}