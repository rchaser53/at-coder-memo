/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 998244353;

fn main() {
  input! {
    h:usize,
    w:usize,
    k:usize,
    h1:usize,
    w1:usize,
    h2:usize,
    w2:usize
  }
  
  // 0, hが同じ
  // 1, wが同じ
  // 2, h,wが同じ
  // 3, h,wが異なる
  let mut result = vec![vec![0,0,0,0];k+1];

  if h1 == h2 && w1 == w2 {
    result[0][2] = 1;
  } else if h1 == h2 {
    result[0][0] = 1;
  } else if w1 == w2 {
    result[0][1] = 1;
  } else {
    result[0][3] = 1;
  }

  for i in 0..k {

    // 0, hが同じ
    result[i+1][0] = result[i][2] * (w-1) % MOD;
    result[i+1][0] += result[i][0] * (w-2) % MOD;
    result[i+1][0] %= MOD;
    result[i+1][0] += result[i][3];
    result[i+1][0] %= MOD;
    
    // 1, wが同じ
    result[i+1][1] = result[i][2] * (h-1) % MOD;
    result[i+1][1] += result[i][1] * (h-2) % MOD;
    result[i+1][1] %= MOD;
    result[i+1][1] += result[i][3];
    result[i+1][1] %= MOD;

    // 2, h,wが同じ
    result[i+1][2] = (result[i][0] + result[i][1]) % MOD;

    // 3, h,wが異なる
    result[i+1][3] += (result[i][0] * (h-1)) % MOD;
    result[i+1][3] %= MOD;
    result[i+1][3] += (result[i][1] * (w-1)) % MOD;
    result[i+1][3] %= MOD;

    result[i+1][3] += (result[i][3] * (h-2)) % MOD;
    result[i+1][3] %= MOD;
    result[i+1][3] += (result[i][3] * (w-2)) % MOD;
    result[i+1][3] %= MOD;
  }

  println!("{}", result[k][2] % MOD);
}