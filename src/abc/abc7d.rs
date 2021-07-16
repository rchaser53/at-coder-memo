/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn helper(mut a:usize) -> usize {
  let mut memo = vec![0;19];
  memo[0] = 2;
  
  for i in 1..=18 {
    let mut temp = 0;
    for j in 0..i {
      temp += memo[j];
    }
    memo[i] = 10usize.pow(i as u32) * 2 + temp * 7;
  }

  let mut left = a % 10;
  let mut result = if left < 4 {
    0
  } else if 4 <= left && left < 9 {
    1
  } else {
    2
  };
  a /= 10;

  for i in 1..=18 {
    if a == 0 { break }
    let base = 10usize.pow(i as u32);
    let re = a % 10;

    let mut temp = 0;
    for j in 0..i {
      temp += memo[j];
    }

    if re < 4 {
      result += re * temp;
    } else if re == 4 {
      result = 4 * temp + left + 1;
    } else if 4 < re && re < 9 {
      result += (re - 1) * temp + base;
    } else {
      result = 8 * temp + base + left + 1;
    }
    left += re * base;
    a /= 10;
  }
  result
}

pub fn main(
) {
  input! {
    a:usize,
    b:usize
  }

  let av = helper(a);
  let bv = helper(b);
  let mut result = bv - av;
  for c in a.to_string().chars() {
    if c == '4' || c == '9' {
      result += 1;
      break
    }
  }

  println!("{}", result);
}
