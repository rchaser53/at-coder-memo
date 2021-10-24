/** OUTPUT **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    n:usize,
    mut vals:[(usize,usize);n],
  }

  vals.sort_by(|a,b| {
    let v = a.1.cmp(&b.1);
    if v == Ordering::Equal {
      a.0.cmp(&b.0)
    } else if v == Ordering::Less {
      Ordering::Greater
    } else {
      Ordering::Less
    }
  });
  println!("{}", vals[n-1].0+vals[n-1].1);
}