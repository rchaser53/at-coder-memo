use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main() {
  input! {
    n:usize,
    k:usize,
    vals:[usize;n]
  }

  let mut map = HashMap::new();
  for i in 0..k {
    *map.entry(vals[i]).or_insert(0) += 1;
  }

  for i in k..n {
    let lv = vals[i-k];
    let nv = vals[i];

    if lv < nv {
      println!("Yes");
    } else {
      println!("No");
    }

    let v = map.get(&lv).unwrap();
    if v == &0 {
      map.remove(&lv);
    }
    *map.entry(nv).or_insert(0) += 1;
  }
}