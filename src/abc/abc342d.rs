/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut a:[usize;n]
  }

  let mut set = HashSet::new();
  for i in 1..=10usize.pow(3) {
    set.insert(i*i);
  }

  let mut map = HashMap::new();
  let mut zero = 0;
  let mut s_num = 0;
  for mut v in a {
    if v == 0 {
      zero += 1;
    } else if set.contains(&v) {
      s_num += 1;
    } else {
      let mut memo = HashMap::new();
      let mut i = 2;
      while i * i <= v {
        if v % i == 0 {
          v /= i;
          *memo.entry(i).or_insert(0) += 1;
        } else {
          i += 1;
        }
      }

      if 1 < v {
        *memo.entry(v).or_insert(0) += 1;
      }

      let mut temp = 1;
      for (key, num) in memo {
        if num % 2 == 1 {
          temp *= key;
        }
      }

      *map.entry(temp).or_insert(0) += 1;
    }
  }

  let mut result = 0;
  if zero >= 1 {
    result += zero * (zero-1) / 2;
    result += zero * (n-zero);
  }
  if s_num >= 1 {
    result += s_num * (s_num-1) / 2;
  }

  for (_, num) in map {
    if num >= 1 {
      result += num * (num-1) / 2;
    }
  }

  println!("{}", result);
}