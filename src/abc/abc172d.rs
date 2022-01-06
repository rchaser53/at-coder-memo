use proconio::input;
use proconio::marker::*;
use std::collections::*;

// 約数の個数を求める
// エラトステネスの ふるい の変形
fn sieve(a: usize) -> Vec<usize> {
  let mut result = vec![1;a+1];
  let mut is_prime = vec![true;a+1];
  for i in 2..=a {
    if is_prime[i] {
      is_prime[i] = false;
      for j in 2..=a/i {
        is_prime[i * j] = false;
        result[i * j] += result[j];
      }
    }
  }

  for i in 2..=a {
    result[i] += 1;
  }
  result
}

fn main() {
  input! {
    n:usize,
  }

  let memo = sieve(n);
  let mut result = 1;
  for i in 2..=n {
    result += i * memo[i];
  }
  println!("{}", result);
}