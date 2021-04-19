#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    a:usize,
    b:usize
  }
  let ia = a as isize;
  let ib = b as isize;

  let mut base = 1500000isize;
  if b < a {
    base /= ia / ib;
  }

  let mut total = 0isize;
  let mut result = vec![0isize;a];
  for i in 0..ia {
    result[i as usize] = base + i;
    total += base + i;
  }
  
  if total % ib != 0 {
    let shim = ib - total % ib;
    result[a-1] += shim;
    total += shim;
  }
  
  let v = -1 * total / ib;
  
  if b % 2 == 1 {
    result.push(v);
  }
  
  for i in 1..=ib/2 {
    result.push(v+i);
    result.push(v-i);
  }

  println!("{}", result
    .into_iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>()
    .join(" ")
  );  
}