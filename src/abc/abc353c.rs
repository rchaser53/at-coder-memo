/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

const MOD:isize = 1_000_000_00;
fn main() {
  input! {
    n:usize,
    mut a:[isize;n],
  }

  a.sort();
  let mut result = 0;
  let mut memo = vec![0;n+1];
  for i in 0..n {
    memo[i+1] = memo[i] + a[i];
  }

  for i in 0..n-1 {
    let v = a[i];
    let mut left = i;
    let mut right = n;
    while left + 1 < right {
      let mid = (left+right)/2;
      let sum = v + a[mid];
      if MOD <= sum {
        right = mid;
      } else {
        left = mid;
      }
    }

    let sum1 = memo[left+1] - memo[i+1];
    let sum2 = memo[n] - memo[left+1];
    let need_minus = (n - left - 1) as isize * MOD;
    let a_i_sum = (n - i - 1) as isize * v;
    result += sum1 + sum2 + a_i_sum - need_minus
  }

  println!("{}", result);
}
