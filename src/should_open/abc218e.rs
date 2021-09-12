use proconio::input;
use proconio::marker::*;
use std::collections::*;

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

// 最小全域木。 クラスカル法
fn clascal(
  edges: &Vec<(isize,isize,isize)>,
  n:usize
) -> isize {
  let mut sum = 0isize;
  let mut memo = vec![-1;n];

  for &(a,b,v) in edges {
    let av = find(&mut memo, a);
    let bv = find(&mut memo, b);
    if av != bv {
      connect(&mut memo, a, b);
    } else if 0 < v {
      sum += v;
    }
  }
  sum
}

fn main() {
  input!{
    n:usize,
    m:usize,
    mut edges:[(Isize1,Isize1,isize);m]
  }
  edges.sort_by(|a,b| a.2.cmp(&b.2));
  println!("{}", clascal(&mut edges, n));
}