/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    mut vals:[Usize1;n]
  }

  let mut count = 0;
  let inf = 1_000_000_000_000;
  for i in 0..n {
    let ti = vals[i];
    if ti == inf { continue }
    if i == vals[ti] {
      count += 1;
      vals[ti] = inf;
    }
  }
  println!("{}", count);
}
