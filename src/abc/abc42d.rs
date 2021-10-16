use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD: usize = 1_000_000_007;

fn modpow(mut a: usize, mut n: usize) -> usize {
  let mut res = 1;
  while n > 0 {
    if n & 1 == 1 {
      res = res * a % MOD; 
    }
    a = a * a % MOD;
    n >>= 1;
  }
  res
}

// a^{-1} mod を計算する
fn mod_inv(a: usize) -> usize {
  modpow(a, MOD-2)
}

// nCk % MOD
// 組み合わせ。2種類の文字の並び替え
// nが大きい時に使う
fn choose(c:usize, k:usize) -> usize {
  let inv_k = mod_inv(k);
  c * inv_k % MOD
}

fn helper(forwards: &Vec<usize>, backs: &Vec<usize>, a:usize, b:usize) -> usize {
  let tot = a + b;
  let rc = backs[tot-a+1];
  let rp = forwards[a];
  let inv_rp = mod_inv(rp);
  rc * inv_rp
}

fn main() {
  input!{
    h:usize,
    w:usize,
    a:usize,
    b:usize
  }

  let tot = h+w;
  let mut forwards = vec![0;tot+1];
  forwards[1] = 1;
  forwards[0] = 1;
  for i in 2..=tot {
    forwards[i] = forwards[i-1] * i % MOD;
  }

  let mut ftv = 1usize;
  let fwv = b-1;
  for i in 0..fwv {
    ftv *= fwv - i;
    ftv %= MOD;
  }
  let fv = choose(ftv, forwards[fwv]) % MOD;

  let mut btv = 1usize;
  let bwv = w-b-1;
  let tot = h-1+bwv;
  for i in 0..bwv {
    btv *= tot - i;
    btv %= MOD;
  }
  let bv = choose(btv, forwards[bwv]);
  let mut result = fv * bv % MOD;

  for i in 1..h-a {
    let tot = fwv + i;
    ftv *= mod_inv(tot-fwv);
    ftv %= MOD;
    ftv *= tot;
    ftv %= MOD;
    let fv = choose(ftv, forwards[fwv]);

    let bhv = h-i-1;
    let tot = bwv+bhv;
    btv *= mod_inv(tot+1);
    btv %= MOD;
    btv *= tot-bwv+1;
    btv %= MOD;
    let bv = choose(btv, forwards[bwv]);

    result += fv * bv % MOD;
    result %= MOD;
  }
  println!("{}", result);
}