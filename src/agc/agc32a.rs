use proconio::input;
use proconio::marker::*;
use std::collections::*;
 
pub fn main(
) {
  input! {
    n:usize,
    mut vals: [usize;n]
  }
 
  let mut result = vec![];


  let mut dirty = true;
  while dirty && !vals.is_empty() {
    dirty = false;

    let mut target = None;
    for i in 0..vals.len() {
      if vals[i] == i + 1 {
        target = Some(i);
      }
    }

    if let Some(ti) = target {
      result.push(vals[ti]);
      dirty = true;
      vals.remove(ti);
    }
  }

  if vals.is_empty() {
    result.reverse();
    for v in result {
      println!("{}", v);
    }
  } else {
    println!("-1");
  }
}