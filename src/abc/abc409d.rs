/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    t:usize,
  }

  for _ in 0..t {
    input! {
      n:usize,
      mut s:Chars,
    }

    let mut memo = vec![-1;26];

    let mut l = n-1;
    let mut r = n-1;
    for i in (0..n).rev() {
      let c = s[i] as usize - 'a' as usize;
      let ii = i as i64;
      memo[c] = memo[c].max(ii);

      // 自分より大きい最小のindexを求める
      let mut min = n;
      for j in c+1..26 {
        if memo[j] != -1 {
          min = min.min(memo[j] as usize);
        }
      }

      // 自分より小さい最大のindexを求める
      let mut max = -1;
      for j in (0..c).rev() {
        if memo[j] != -1 {
          max = memo[j];
        }
      }

      if max == -1 {
        continue
      }
      l = i;

      if min < max as usize && 1 < min {
        r = min;
      } else if (max as usize) < min {
        r = max as usize + 1;
      }
    }

    if l != n - 1 {
      // println!("l: {}, r: {} s:{}", l, r,  s.iter().collect::<String>());
      s.insert(r, s[l]);
      s.remove(l);
    }

    println!("{}", s.iter().collect::<String>());
  }
}