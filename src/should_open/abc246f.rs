/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

const MOD:usize = 998244353;

// n^pを計算するやつ
fn repeat_square(n:usize, p:usize) -> usize {
  if p == 0 {
    1
  } else if p % 2 == 0 {
    repeat_square(n, p/2).pow(2) % MOD 
  } else {
    n * repeat_square(n, p-1) % MOD
  }
}

fn main() {
  input! {
    n:usize,
    l:usize,
    ss:[Chars;n]
  }

  let limit = 1 << n;
  let mut memo = vec![0;n];
  for i in 0..n {
    let mut temp = 0;
    for &c in &ss[i] {
      let v = (c as u8 - 'a' as u8) as usize;
      temp |= 1 << v;
    }
    memo[i] = temp;
  }

  // 包除原理
  // これだとinput1~nでそれそれ作成できる文字列の個数を重複なく求めることができる
  let mut result = 0usize;
  for i in 1usize..limit {
    let mut temp = (1<<26)-1usize;
    for j in 0..n {
      if i >> j & 1 == 1 {
        temp &= memo[j];
      }
    }
    let count = temp.count_ones() as usize;
    let v = repeat_square(count, l);
    if i.count_ones() % 2 == 1 {
      result += v;
      result %= MOD;
    } else {
      result += MOD;
      result -= v;
      result %= MOD;
    }
  }

  println!("{}", result);
}