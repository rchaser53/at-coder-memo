/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    mut vals:[isize;n],
  }
  vals.sort();
  let mut result = vals.pop().unwrap();
  let mut first = vals[0];

  let mut processes = vec![];
  for i in 1..vals.len() {
    let v = vals[i];
    if v < 0 {
      processes.push((result, v));
      result -= v;
    } else {
      processes.push((first, v));
      first -= v;
    }
  }
  processes.push((result, first));
  result -= first;

  println!("{}", result);
  for (l, r) in processes {
    println!("{} {}", l, r);
  }
}