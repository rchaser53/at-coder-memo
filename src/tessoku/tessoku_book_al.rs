/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! { 
    d:usize,
    n:usize,
    vs:[(Usize1,usize,usize);n]
  }

  let mut adds = vec![vec![];d+1];
  let mut removes = vec![vec![];d+1];
  for i in 0..n {
    let (a,b,c) = vs[i];
    adds[a].push((c,i));
    removes[b].push((c,i));
  }

  let mut btreeset = BTreeSet::new();
  let mut result = 0usize;
  for i in 0..d {
    for add in &adds[i] {
      btreeset.insert(add);
    }
    for remove in &removes[i] {
      btreeset.remove(&remove);
    }

    if let Some((v, _)) = btreeset.iter().next() {
      result += v;
    } else {
      result += 24;
    }
  }

  println!("{}", result);
}