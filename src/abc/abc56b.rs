use proconio::input;
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    w:isize,
    a:isize,
    b:isize,
  }

  if a <= b && b <= a + w {
    println!("0");
    return
  } else if a <= b+w && b <= a + w {
    println!("0");
    return
  }

  let rv = (a + w - b).abs();
  let lv = (b + w - a).abs();
  println!("{}", std::cmp::min(rv, lv));
}
