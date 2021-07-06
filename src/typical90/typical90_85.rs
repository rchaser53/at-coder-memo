/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    k:usize,
  }

  let mut set = HashSet::new();
  let mut i = 1;
  while i * i <= k {
    if k % i == 0 {
      set.insert(i);
      set.insert(k/i);
    }
    i += 1;
  }

  let mut arr = set.into_iter().collect::<Vec<_>>();
  let mut sorted = vec![0,0,0];

  let mut result = HashSet::new();
  arr.sort();
  for &a in &arr {
    if k % a != 0 { continue }
    let v1 = k / a;
    if v1 == 0 { continue }
    for &b in &arr {
      if v1 % b != 0 { continue }
      let c = v1 / b;
      if c == 0 { continue }
      sorted[0] = a;
      sorted[1] = b;
      sorted[2] = c;
      sorted.sort();
      result.insert((sorted[0], sorted[1], sorted[2]));
    }
  }

  println!("{}", result.len());
}
