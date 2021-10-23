/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

// 高速に素因数分解するのに使える
// エラトステネスの変形
fn sieve(a: usize) -> Vec<usize> {
  let mut result = vec![1;a+1];
  let mut is_prime = vec![true;a+1];
  for i in 2..a+1 {
    if is_prime[i] {
      result[i] = i;
      for j in 1..a / i + 1 {
        is_prime[i * j] = false;
        result[i * j] = i;
      }
    }
  }
  result
}

fn gcd(a:usize, b:usize) -> usize {
  if a == 0 {
    return b
  }
  gcd(b % a, a)
}

pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n]
  }

  let mut gv = vals[0];
  for &v in &vals {
    gv = gcd(v, gv);
  }
  if gv != 1 {
    println!("not coprime");
    return
  }

  let primes = sieve(10usize.pow(6)+3);
  let mut set = HashSet::new();
  for &v in &vals {
    let mut v = v;

    while 1 < v {
      let lv = primes[v]; 
      v = primes[v];
      if v == 1 { break }
      if set.contains(&v) {
        println!("setwise coprime");
        return
      } else {
        set.insert(v);
      }
      if v == lv { break }
    }
  }
  println!("pairwise coprime");
}
