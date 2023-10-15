/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

const MOD:usize = 998244353;
fn mod_pow(a: usize, n: usize) -> usize { 
  if n == 0 {
    1
  } else if n == 1 {
    a % MOD
  } else if n % 2 == 1 {
    let k = mod_pow(a, (n - 1) / 2);
    let ans = (a * k) % MOD;
    (ans * k) % MOD
  } else {
    let k = mod_pow(a, n / 2);
    (k * k) % MOD
  }
}

// a^{-1} mod を計算する
fn mod_inv(a:usize) -> usize {
  mod_pow(a, MOD-2)
}

fn main() {
  input! {
    n:usize,
    x:usize,
    t:[usize;n]
  }

  let limit = x+10;
  let mut memo = vec![0;limit];
  memo[0] = 1;

  let t0v = t[0];
  let mut map = HashMap::new();
  for v in t {
    *map.entry(v).or_insert(0) += 1usize;
  }
  let mod_inv_n = mod_inv(n);

  let arr = map.into_iter().map(|(a,b)| (a,b)).collect::<Vec<(usize,usize)>>();
  let mut result = 0;
  for i in 0..=x {
    let now = memo[i];
    if x < i + t0v {
      result += now * mod_inv_n % MOD;
      result %= MOD;
    }

    for (ai, num) in &arr {
      let ni = i + ai;
      if limit <= ni { continue }
      let av = now * num % MOD * mod_inv_n % MOD;
      memo[ni] += av;
      memo[ni] %= MOD;
    }  
  }

  println!("{}", result);
}