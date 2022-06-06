/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

const MOD:usize = 1_000_000_007;
const RANGE:usize = 27;
fn helper(
  s:&Vec<usize>, memo: &Vec<usize>,
  start:usize, len:usize, expect:usize
) -> bool {
  let n = s.len();
  if n <= start + len - 1 { return false }

  let mut base = 0;
  for i in start..start+len {
    base = (base * RANGE + s[i] + 1) % MOD;
  }

  if base == expect { return true }
  for i in start+len..n {
    let li = i - len;
    base = (base * RANGE + s[i] + 1) % MOD;
    base = (base + MOD - memo[s[li]]) % MOD;

    if base == expect { return true }
  }
  false
}

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
    s:String,
  }

  let s = s.chars().map(|v| (v as u8 - 'a' as u8) as usize).collect::<Vec<usize>>();
  let n = s.len();
  let mut left = 0;
  let mut right = n/2+1;
  while left+1 < right {
    let mid = (left+right) / 2;
    let mut memo = vec![0;RANGE-1];
    for i in 0..26 {
      memo[i] = (i+1) * repeat_square(RANGE, mid) % MOD;
    }

    // i == 0
    let mut expect = 0usize;
    for i in 0..mid {
      expect = (expect * RANGE + s[i] + 1) % MOD;
    }

    let mut success = false;
    if helper(&s, &memo, mid, mid, expect) {
      success = true;
    } else {
      
      for i in mid..n {
        let li = i - mid;
        expect = (expect * RANGE + s[i]+1) % MOD;
        expect = (expect + MOD - memo[s[li]]) % MOD;        
        if helper(&s, &memo, i+1, mid, expect) {
          success = true;
          break
        }
      }
    }

    if success {
      left = mid;
    } else {
      right = mid;
    }
  }

  println!("{}", left);
}