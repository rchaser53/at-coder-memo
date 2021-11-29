/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
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
  input! {
    n:usize,
    m:usize,
    points:[(Isize1, Isize1);m],
  }

  let mut tree = vec![-1;n];
  let mut g = vec![vec![];n];
  for (f, t) in points {
    g[f as usize].push(t);
    g[t as usize].push(f);
  }

  let mut result = vec![0;n];
  let mut now = 0;
  for i in (0..n).rev() {
    result[i] = now;
    let ii = i as isize;
    for &j in &g[i] {
      if j < ii { continue }

      if find(&mut tree, ii) == find(&mut tree, j) {
        continue
      }
      now -= 1;

      connect(&mut tree, ii, j);
    }

    now += 1;
  }

  for v in result {
    println!("{}", v);
  }
}