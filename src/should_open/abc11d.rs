/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n: usize,
    d: usize,
    x: isize,
    y: isize,
  }
  
  let x = x.abs() as usize;
  let y = y.abs() as usize;
  if x % d != 0 || y % d != 0 {
    println!("0");
    return
  }

  let x = x / d;
  let y = y / d;
  if n < x + y || (n-x-y) % 2 == 1 {
    println!("0");
    return
  }

  let limit = n + 1;
  let mut memo = vec![vec![0f64;limit];limit];
  memo[0][0] = 1f64;

  for i in 1..=n {
    memo[i][0] = memo[i-1][0] / 2f64;
    for j in 1..=i {
      memo[i][j] = (memo[i-1][j] + memo[i-1][j-1]) / 2f64;
    }
  }

  let mut i = 0;
  let mut result = 0f64;
  while x + (i*2) + y <= n {
    let xtot = x + (i * 2);
    let xk = i;
    let left = n - xtot - y;
    let ytot = y + left;
    let yk = left / 2;

    let mut temp = memo[n][xtot];
    temp *= memo[xtot][xk];
    temp *= memo[ytot][yk];
    result += temp;
    i += 1;
  }

  println!("{}", result);
}
