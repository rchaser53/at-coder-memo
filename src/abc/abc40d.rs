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

fn main() {
  input!{
    n:usize,
    m:usize,
    mut edges:[(Isize1, Isize1, usize);m],
    q:usize,
    mut vals: [(Isize1,usize);q]
  }

  let mut memo = vec![-1;n];
  edges.sort_by(|a,b| a.2.cmp(&b.2));
  edges.reverse();

  let mut vals = vals.into_iter().enumerate().collect::<Vec<(usize,(isize,usize))>>();
  vals.sort_by(|a,b| (a.1).1.cmp(&(b.1).1));
  vals.reverse();
  let mut result = vec![0;q];

  let mut ei = 0;
  for (i, (live_town_index, val)) in vals {
    while ei < m && val < edges[ei].2 {
      let (a, b, should_large) = edges[ei];
      connect(&mut memo, a, b);
      ei += 1;
    }
    result[i] = size(&mut memo, live_town_index);
  }
  
  for v in result {
    println!("{}", v);
  }
}